#!/usr/bin/env python3
"""
Hudi to ClickHouse Data Sync Utility

This script reads data from Hudi tables and syncs it to ClickHouse.
It can be run as a background process or scheduled job.
"""

import sys
import os
import logging
from datetime import datetime, timedelta
from pyspark.sql import SparkSession
from pyspark.sql.functions import col, lit
import clickhouse_connect

# Add project root to path
sys.path.append('/media/sandeep/DataDrive/galactus')

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('/media/sandeep/DataDrive/galactus/logs/hudi_clickhouse_sync.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

class HudiClickHouseSync:
    def __init__(self, hudi_base_path="/tmp/hudi_data", clickhouse_host="localhost", clickhouse_port=8123):
        self.hudi_base_path = hudi_base_path
        self.clickhouse_host = clickhouse_host
        self.clickhouse_port = clickhouse_port

        # Initialize ClickHouse client
        self.ch_client = clickhouse_connect.get_client(
            host=self.clickhouse_host,
            port=self.clickhouse_port,
            database='galactus'
        )

        # Initialize Spark session
        self.spark = self._create_spark_session()
        # Ensure ClickHouse table exists (created on first sync if missing)
        self._ensure_table_exists()

    def _create_spark_session(self):
        """Create Spark session with Hudi support"""
        return SparkSession.builder \
            .appName("Hudi-ClickHouse-Sync") \
            .config("spark.serializer", "org.apache.spark.serializer.KryoSerializer") \
            .config("spark.sql.hive.convertMetastoreParquet", "false") \
            .getOrCreate()

    def read_hudi_table(self, table_name, base_path=None):
        """Read data from Hudi table"""
        if base_path is None:
            base_path = f"{self.hudi_base_path}/{table_name}"

        logger.info(f"Reading Hudi table from: {base_path}")

        try:
            df_raw = self.spark.read.format("hudi").load(base_path)

            # normalize column names to lowercase
            df = df_raw.select([col(c).alias(c.lower()) for c in df_raw.columns])

            # canonicalize some known column name variants
            if 'ttl_trd_qnty' in df.columns and 'total_traded_qty' not in df.columns:
                df = df.withColumnRenamed('ttl_trd_qnty', 'total_traded_qty')
            if 'turnover_lacs' not in df.columns and 'turnover_lac' in df.columns:
                df = df.withColumnRenamed('turnover_lac', 'turnover_lacs')

            # Ensure required columns exist (will be None if missing)
            expected = [
                'symbol', 'series', 'open_price', 'high_price', 'low_price',
                'close_price', 'last_price', 'prev_close', 'total_traded_qty',
                'turnover_lacs', 'date1', 'no_of_trades', 'record_key',
                'trade_date', '_hoodie_commit_time', '_hoodie_commit_seqno',
                '_hoodie_record_key', '_hoodie_partition_path', '_hoodie_file_name'
            ]

            # select existing columns, add nulls for missing ones
            select_cols = []
            for c in expected:
                if c in df.columns:
                    select_cols.append(col(c))
                else:
                    select_cols.append(lit(None).alias(c))

            df = df.select(*select_cols)
            logger.info(f"Successfully read {df.count()} records from Hudi")
            return df
        except Exception as e:
            logger.error(f"Error reading Hudi table: {e}")
            return None

    def sync_to_clickhouse(self, df, table_name):
        """Sync DataFrame to ClickHouse with upsert logic using symbol + trade_date as unique key"""
        if df is None or df.count() == 0:
            logger.warning("No data to sync")
            return False

        try:
            # Stream rows from Spark to avoid building a full pandas DataFrame.
            batch_size = 1000
            data_batch = []
            total = 0

            def _convert_row(row):
                # Helper: safely extract attributes from Row
                def gv(k):
                    return row[k] if k in row and row[k] is not None else None

                # Parse DATE1 -> datetime if possible
                date_str = gv('DATE1')
                ts = None
                if date_str:
                    try:
                        ts = datetime.strptime(str(date_str), "%d-%b-%Y")
                    except Exception:
                        try:
                            ts = datetime.strptime(str(date_str), "%Y-%m-%d")
                        except Exception:
                            ts = datetime.now()

                # Parse trade_date -> date
                trade_date = None
                td = gv('trade_date')
                if td:
                    try:
                        trade_date = datetime.strptime(str(td), "%Y-%m-%d").date()
                    except Exception:
                        try:
                            trade_date = datetime.strptime(str(td), "%d-%b-%Y").date()
                        except Exception:
                            trade_date = datetime.now().date()

                return [
                    str(gv('symbol') or ''),
                    str(gv('series') or ''),
                    float(gv('open_price') or 0.0),
                    float(gv('high_price') or 0.0),
                    float(gv('low_price') or 0.0),
                    float(gv('close_price') or 0.0),
                    float(gv('last_price') or 0.0),
                    float(gv('prev_close') or 0.0),
                    int(gv('total_traded_qty') or 0),
                    float(gv('turnover_lacs') or 0.0),
                    ts,
                    int(gv('no_of_trades') or 0),
                    str(gv('record_key') or ''),
                    trade_date,
                    str(gv('_hoodie_commit_time') or ''),
                    str(gv('_hoodie_commit_seqno') or ''),
                    str(gv('_hoodie_record_key') or ''),
                    str(gv('_hoodie_partition_path') or ''),
                    str(gv('_hoodie_file_name') or '')
                ]

            try:
                for row in df.toLocalIterator():
                    data_batch.append(_convert_row(row.asDict()))
                    if len(data_batch) >= batch_size:
                        self._flush_batch(table_name, data_batch)
                        total += len(data_batch)
                        data_batch = []

                # Final batch
                if data_batch:
                    self._flush_batch(table_name, data_batch)
                    total += len(data_batch)

                logger.info(f"Successfully upserted ~{total} records to ClickHouse table {table_name}")
                return True

            except Exception as e:
                logger.error(f"Error syncing to ClickHouse while streaming: {e}")
                return False

        except Exception as e:
            logger.error(f"Error in sync_to_clickhouse: {e}")
            return False

    def sync_table(self, table_name, base_path=None):
        """Sync a complete table from Hudi to ClickHouse"""
        logger.info(f"Starting sync for table: {table_name}")

        # Read from Hudi
        df = self.read_hudi_table(table_name, base_path)
        if df is None:
            return False

        # Sync to ClickHouse
        success = self.sync_to_clickhouse(df, table_name)

        if success:
            logger.info(f"Sync completed successfully for table: {table_name}")
        else:
            logger.error(f"Sync failed for table: {table_name}")

        return success

    def incremental_sync(self, table_name, last_sync_time=None, base_path=None):
        """Perform incremental sync based on commit time"""
        if base_path is None:
            base_path = f"{self.hudi_base_path}/{table_name}"

        logger.info(f"Starting incremental sync for table: {table_name}")

        try:
            # Read Hudi table with time travel if last_sync_time is provided
            df = self.spark.read.format("hudi") \
                .option("as.of.instant", last_sync_time) \
                .load(base_path) \
                .select(
                    "symbol", "series", "open_price", "high_price", "low_price",
                    "close_price", "last_price", "prev_close", "total_traded_qty",
                    "total_traded_val", "timestamp", "total_trades", "isin",
                    "trade_date", "_hoodie_commit_time", "_hoodie_commit_seqno",
                    "_hoodie_record_key", "_hoodie_partition_path", "_hoodie_file_name"
                )

            # Filter for records newer than last sync
            if last_sync_time:
                df = df.filter(col("_hoodie_commit_time") > last_sync_time)

            logger.info(f"Incremental sync: {df.count()} new records")
            return self.sync_to_clickhouse(df, table_name)

        except Exception as e:
            logger.error(f"Error in incremental sync: {e}")
            return False

    def close(self):
        """Clean up resources"""
        if self.spark:
            self.spark.stop()
        if self.ch_client:
            self.ch_client.close()

    def _flush_batch(self, table_name, data_batch):
        """Insert a batch of rows into ClickHouse. Uses simple delete+insert upsert pattern for conflicting keys."""
        if not data_batch:
            return

        try:
            # Attempt a delete of existing rows for the symbol+trade_date pairs in batch
            symbols_and_dates = [(r[0], r[13]) for r in data_batch if r[0] and r[13]]
            if symbols_and_dates:
                delete_conditions = []
                for symbol, trade_date in symbols_and_dates:
                    delete_conditions.append(f"(symbol = '{symbol}' AND trade_date = '{trade_date}')")
                if delete_conditions:
                    delete_query = f"DELETE FROM {table_name} WHERE {' OR '.join(delete_conditions)}"
                    try:
                        self.ch_client.command(delete_query)
                    except Exception:
                        # ignore delete failures and proceed to insert
                        pass

            # Insert batch
            self.ch_client.insert(table_name, data_batch,
                                  column_names=[
                                      'symbol', 'series', 'open_price', 'high_price', 'low_price',
                                      'close_price', 'last_price', 'prev_close', 'total_traded_qty',
                                      'turnover_lacs', 'timestamp', 'total_trades', 'isin',
                                      'trade_date', '_hoodie_commit_time', '_hoodie_commit_seqno',
                                      '_hoodie_record_key', '_hoodie_partition_path', '_hoodie_file_name'
                                  ])
        except Exception as e:
            logger.error(f"Batch insert failed: {e}")

    def _ensure_table_exists(self):
        """Create ClickHouse table if it does not exist with a compatible schema."""
        try:
            create_sql = (
                "CREATE TABLE IF NOT EXISTS galactus.sec_bhavdata ("
                "symbol String, series String, open_price Float64, high_price Float64, low_price Float64, "
                "close_price Float64, last_price Float64, prev_close Float64, total_traded_qty UInt64, "
                "turnover_lacs Float64, timestamp DateTime, total_trades UInt32, isin String, trade_date Date, "
                "_hoodie_commit_time String, _hoodie_commit_seqno String, _hoodie_record_key String, "
                "_hoodie_partition_path String, _hoodie_file_name String) "
                "ENGINE = MergeTree() ORDER BY (trade_date, symbol)"
            )
            self.ch_client.command(create_sql)
            logger.info("Ensured ClickHouse table galactus.sec_bhavdata exists")
        except Exception as e:
            logger.error(f"Could not ensure ClickHouse table exists: {e}")

def main():
    import argparse

    parser = argparse.ArgumentParser(description='Sync Hudi tables to ClickHouse')
    parser.add_argument('--table', required=True, help='Table name to sync')
    parser.add_argument('--hudi-path', default='/tmp/hudi_data', help='Base path for Hudi tables')
    parser.add_argument('--incremental', action='store_true', help='Perform incremental sync')
    parser.add_argument('--last-sync-time', help='Last sync timestamp for incremental sync')

    args = parser.parse_args()

    sync = HudiClickHouseSync(hudi_base_path=args.hudi_path)

    try:
        if args.incremental:
            success = sync.incremental_sync(args.table, args.last_sync_time)
        else:
            success = sync.sync_table(args.table)

        sys.exit(0 if success else 1)

    except Exception as e:
        logger.error(f"Sync failed: {e}")
        sys.exit(1)
    finally:
        sync.close()

if __name__ == "__main__":
    main()