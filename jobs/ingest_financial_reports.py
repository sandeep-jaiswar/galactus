"""Spark job to ingest financial reports into a Hudi table.

This is a scaffolded job that mirrors the bhavcopy ingestion pattern:
- query announcements (downloader)
- download raw documents
- parse and normalize
- write to Hudi

Arguments (basic):
  --date YYYY-MM-DD (single date) or --start/--end for ranges
  --hudi-path <path>
"""
import argparse
import os
from pyspark.sql import SparkSession
from pyspark.sql.functions import col
from typing import List
import tempfile
import shutil
import os
from datetime import date
import json
import clickhouse_connect
import pandas as pd

# Local imports
from utils.financial_results import get_financial_results_master, financial_results_for_equity
from utils.financial_parser import normalize
from conf.hudi import hudi_write_options
from utils.hudi_clickhouse_financials import HudiClickHouseFinancials




def build_spark(app_name: str = "ingest_financial_reports") -> SparkSession:
    return SparkSession.builder.appName(app_name).getOrCreate()


def run_for_date(spark: SparkSession, date_str: str, hudi_base_path: str, symbols=None, dry_run: bool = False,
                 sync_clickhouse: bool = False, clickhouse_cfg: dict = None, python_mode: bool = False):
    # NSE API expects dd-mm-YYYY — convert date_str if provided as YYYY-MM-DD
    def to_nse_date(ds: str) -> str:
        if not ds:
            return ds
        parts = ds.split("-")
        if len(parts) == 3 and len(parts[0]) == 4:
            # YYYY-MM-DD -> DD-MM-YYYY
            return f"{parts[2]}-{parts[1]}-{parts[0]}"
        return ds

    nse_date = to_nse_date(date_str)

    # Use the financial results pipeline to fetch and parse XBRL-backed results for this date
    if dry_run:
        master_df, headers, ns, keys = get_financial_results_master(nse_date, nse_date)
        print(f"Master rows for {date_str}: {len(master_df)}")
        return

    parsed_df = financial_results_for_equity(nse_date, nse_date)
    if parsed_df is None or parsed_df.empty:
        print(f"No parsed financial records for {date_str}")
        return

    # Ensure canonical fields and add record key
    records = []
    for r in parsed_df.to_dict(orient='records'):
        rec = normalize(r)
        # Keep metrics and other fields from parsed result
        rec["metrics"] = r.get("metrics", {})
        rec["raw"] = r.get("raw")
        rec["source_url"] = r.get("source_url")
        # Determine announcement date (use report_date if available)
        ann_date = rec.get("report_date") or None
        if isinstance(ann_date, str) and len(ann_date) >= 10:
            ann_iso = ann_date[:10]
        else:
            ann_iso = None
        # Classify result as annual vs quarterly
        def _classify(rec):
            rt = (rec.get('report_type') or '')
            rt_l = rt.lower() if isinstance(rt, str) else ''
            if 'annual' in rt_l or 'year' in rt_l:
                return 'annual'
            # try period_end month detection
            pe = rec.get('period_end')
            try:
                if isinstance(pe, str) and len(pe) >= 10:
                    from datetime import datetime
                    dt = datetime.fromisoformat(pe[:10])
                    if dt.month == 3 and dt.day == 31:
                        return 'annual'
                # fallback: if reporting quarter present and contains Q
                if isinstance(rt, str) and ('q' in rt_l or 'quarter' in rt_l):
                    return 'quarter'
            except Exception:
                pass
            return 'quarter'

        rec_type = _classify(rec)
        rec['result_type'] = rec_type
        rec['announcement_date'] = ann_iso or None
        # partition_path for Hudi write
        symbol_safe = (rec.get('symbol') or 'UNKNOWN').replace('/', '_')
        part_date = rec['announcement_date'] or ''
        rec['partition_path'] = f"symbol={symbol_safe}/announcement_date={part_date}/result_type={rec_type}"
        # Fiscal year
        if not rec.get("fiscal_year") and rec.get("period_end"):
            try:
                rec["fiscal_year"] = int(str(rec.get("period_end"))[:4])
            except Exception:
                rec["fiscal_year"] = None
        # ingestion timestamp used as Hudi precombine ordering field
        try:
            from datetime import datetime
            rec['_ingest_time'] = datetime.utcnow().isoformat()
        except Exception:
            rec['_ingest_time'] = None
        # record key
        # ensure Hudi precombine field 'report_date' is populated (fallback to ingest time)
        if not rec.get('report_date'):
            rec['report_date'] = rec.get('_ingest_time')
        period_str = rec.get("period_end") or ""
        rec["_record_key"] = f"{rec.get('symbol')}_{rec.get('report_type')}_{period_str}"
        records.append(rec)

    if not records:
        print("No records to write for", date_str)
        return

    # Convert to Spark DataFrame
    if python_mode:
        # Write Parquet files partitioned into two logical tables: quarter_results, annual_results
        out_df = pd.DataFrame(records)
        out_root = os.path.join(hudi_base_path, "python_mode")
        os.makedirs(out_root, exist_ok=True)

        # Split into tables
        quarter_df = out_df[out_df['result_type'] == 'quarter'] if 'result_type' in out_df.columns else out_df
        annual_df = out_df[out_df['result_type'] == 'annual'] if 'result_type' in out_df.columns else out_df.iloc[0:0]

        def _write_partitioned(df, table_name):
            if df is None or df.empty:
                return 0
            written = 0
            for _, row in df.iterrows():
                sym = str(row.get('symbol') or 'UNKNOWN').replace('/', '_')
                ann = row.get('announcement_date') or ''
                rtype = row.get('result_type') or ''
                part_dir = os.path.join(out_root, table_name, f"symbol={sym}", f"announcement_date={ann}", f"result_type={rtype}")
                os.makedirs(part_dir, exist_ok=True)
                file_path = os.path.join(part_dir, f"record_{abs(hash(str(row.get('_record_key') or row)))%100000}.parquet")
                try:
                    # prefer parquet when available
                    try:
                        pd.DataFrame([row]).to_parquet(file_path, index=False)
                        written += 1
                        continue
                    except Exception:
                        # fallback to json lines
                        pass
                    json_path = file_path.replace('.parquet', '.json')
                    with open(json_path, 'w', encoding='utf-8') as jf:
                        jf.write(pd.DataFrame([row]).to_json(orient='records'))
                    written += 1
                except Exception as e:
                    print(f"Failed to write record to {file_path} or fallback json: {e}")
            return written

        q_written = _write_partitioned(quarter_df, 'quarter_results')
        a_written = _write_partitioned(annual_df, 'annual_results')
        print(f"Wrote {q_written} quarterly and {a_written} annual records under {out_root}")

        if sync_clickhouse:
            # Use HudiClickHouseFinancials to create tables if missing and batch upsert
            cfg = clickhouse_cfg or {}
            ch_host = cfg.get('host', 'localhost')
            ch_port = int(cfg.get('port', 8123))
            ch_db = cfg.get('database', 'galactus')
            ch_user = cfg.get('user')
            ch_pass = cfg.get('password')
            ch_table = cfg.get('table', 'company_financials')
            try:
                ch_helper = HudiClickHouseFinancials(host=ch_host, port=ch_port, database=ch_db, user=ch_user, password=ch_pass)
                if q_written:
                    q_rows = quarter_df.where(pd.notnull(quarter_df), None).to_dict(orient='records')
                    inserted_q = ch_helper.batch_upsert(f"{ch_table}_quarter", q_rows)
                    print(f"Inserted {inserted_q} rows into ClickHouse {ch_db}.{ch_table}_quarter")
                if a_written:
                    a_rows = annual_df.where(pd.notnull(annual_df), None).to_dict(orient='records')
                    inserted_a = ch_helper.batch_upsert(f"{ch_table}_annual", a_rows)
                    print(f"Inserted {inserted_a} rows into ClickHouse {ch_db}.{ch_table}_annual")
                ch_helper.client.close()
            except Exception as e:
                print(f"ClickHouse upsert failed in python-mode: {e}")

        return

    # Spark path (default) — convert to Spark DataFrame via pandas to handle nested dicts
    # Sanitize records so Spark can infer a consistent schema
    def _sanitize_value(v):
        from datetime import date, datetime
        if v is None:
            return None
        if isinstance(v, (str, int, float, bool)):
            return v
        if isinstance(v, (date, datetime)):
            return v.isoformat()
        try:
            # convert dicts/lists to JSON
            return json.dumps(v)
        except Exception:
            return str(v)

    sanitized = []
    for r in records:
        s = {}
        for k, v in r.items():
            if k == 'metrics' and isinstance(v, dict):
                s[k] = json.dumps(v)
            else:
                s[k] = _sanitize_value(v)
        sanitized.append(s)

    # Create DataFrame with explicit string schema to avoid inference issues
    from pyspark.sql.types import StructType, StructField, StringType
    all_keys = set()
    for s in sanitized:
        all_keys.update(s.keys())
    schema = StructType([StructField(k, StringType(), True) for k in sorted(all_keys)])
    df = spark.createDataFrame(sanitized, schema=schema)

    # Split and write to Hudi tables per result_type
    record_key = "_record_key"
    precombine_key = "_ingest_time"
    partition_key = "partition_path"

    # Quarter table
    try:
        df_q = df.filter(col('result_type') == 'quarter')
        if df_q.count() > 0:
            table_q = 'quarter_results'
            opts_q = hudi_write_options(table_q, record_key, precombine_key, partition_key)
            table_path_q = os.path.join(hudi_base_path, table_q)
            print(f"Writing {df_q.count()} quarterly records to Hudi at {table_path_q}")
            (df_q.write.format('hudi').options(**opts_q).mode('append').save(table_path_q))
    except Exception as e:
        print(f"Failed to write quarter_results to Hudi: {e}")

    # Annual table
    try:
        df_a = df.filter(col('result_type') == 'annual')
        if df_a.count() > 0:
            table_a = 'annual_results'
            opts_a = hudi_write_options(table_a, record_key, precombine_key, partition_key)
            table_path_a = os.path.join(hudi_base_path, table_a)
            print(f"Writing {df_a.count()} annual records to Hudi at {table_path_a}")
            (df_a.write.format('hudi').options(**opts_a).mode('append').save(table_path_a))
    except Exception as e:
        print(f"Failed to write annual_results to Hudi: {e}")

    # Optionally sync to ClickHouse
    if sync_clickhouse:
        cfg = clickhouse_cfg or {}
        ch_host = cfg.get('host', 'localhost')
        ch_port = int(cfg.get('port', 8123))
        ch_db = cfg.get('database', 'galactus')
        ch_user = cfg.get('user')
        ch_pass = cfg.get('password')
        ch_table = cfg.get('table', 'company_financials')
        try:
            # Use records (already in memory) to upsert into ClickHouse tables
            ch_helper = HudiClickHouseFinancials(host=ch_host, port=ch_port, database=ch_db, user=ch_user, password=ch_pass)
            # split records
            q_records = [r for r in records if r.get('result_type') == 'quarter']
            a_records = [r for r in records if r.get('result_type') == 'annual']
            if q_records:
                n_q = ch_helper.batch_upsert(f"{ch_table}_quarter", q_records)
                print(f"Inserted {n_q} quarter rows into ClickHouse {ch_db}.{ch_table}_quarter")
            if a_records:
                n_a = ch_helper.batch_upsert(f"{ch_table}_annual", a_records)
                print(f"Inserted {n_a} annual rows into ClickHouse {ch_db}.{ch_table}_annual")
            ch_helper.client.close()
        except Exception as e:
            print(f"ClickHouse sync failed: {e}")


def main(argv: List[str]):
    parser = argparse.ArgumentParser()
    parser.add_argument("--date", help="Single date (YYYY-MM-DD)")
    parser.add_argument("--start", help="Start date (YYYY-MM-DD)")
    parser.add_argument("--end", help="End date (YYYY-MM-DD)")
    parser.add_argument("--symbol", help="Single symbol to run for (e.g. GAEL)")
    parser.add_argument("--hudi-path", required=True, help="Base path for Hudi table")
    parser.add_argument("--dry-run", action="store_true", help="Do not download or write, only list announcements")
    parser.add_argument("--python-mode", action="store_true", help="Run pure-Python pipeline (Parquet + optional ClickHouse) without Spark/Hudi")
    parser.add_argument("--sync-clickhouse", action="store_true", help="After write, sync to ClickHouse (python-mode or Spark mode)")
    parser.add_argument("--clickhouse-host", default='localhost', help="ClickHouse host")
    parser.add_argument("--clickhouse-port", default=8123, type=int, help="ClickHouse port")
    parser.add_argument("--clickhouse-database", default='galactus', help="ClickHouse database")
    parser.add_argument("--clickhouse-table", default='company_financials', help="ClickHouse table name")
    args = parser.parse_args(argv)

    # Build list of date strings to run
    dates = []
    if args.date:
        dates = [args.date]
    elif args.start and args.end:
        from datetime import datetime, timedelta

        def _parse(d):
            return datetime.strptime(d, "%Y-%m-%d").date()

        start_d = _parse(args.start)
        end_d = _parse(args.end)
        cur = start_d
        while cur <= end_d:
            dates.append(cur.isoformat())
            cur = cur + timedelta(days=1)
    else:
        # default to today
        from datetime import date as _d

        dates = [_d.today().isoformat()]

    symbols = [args.symbol] if args.symbol else None

    clickhouse_cfg = {
        'host': args.clickhouse_host,
        'port': args.clickhouse_port,
        'database': args.clickhouse_database,
        'table': args.clickhouse_table,
    }

    if args.python_mode:
        # run in pure-Python mode (no Spark)
        for d in dates:
            run_for_date(None, d, args.hudi_path, symbols=symbols, dry_run=args.dry_run,
                         sync_clickhouse=args.sync_clickhouse, clickhouse_cfg=clickhouse_cfg, python_mode=True)
        return

    # Default: Spark-based path
    spark = build_spark()
    for d in dates:
        run_for_date(spark, d, args.hudi_path, symbols=symbols, dry_run=args.dry_run,
                     sync_clickhouse=args.sync_clickhouse, clickhouse_cfg=clickhouse_cfg)

    spark.stop()


if __name__ == "__main__":
    import sys

    main(sys.argv[1:])
