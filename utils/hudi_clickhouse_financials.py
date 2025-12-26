"""ClickHouse sync helper for financials.

Implements table creation and batched upsert for normalized financial records.
Uses `clickhouse_connect` client.
"""
from typing import Dict, Iterable, List, Optional, Any
from datetime import datetime
import json

import clickhouse_connect


class HudiClickHouseFinancials:
    def __init__(self, host: str = 'localhost', port: int = 8123, database: str = 'galactus', user: str = None,
                 password: str = None, client: Optional[Any] = None):
        if client:
            self.client = client
        else:
            # lazily create client
            try:
                self.client = clickhouse_connect.get_client(host=host, port=port, username=user or '', password=password or '', database=database)
            except Exception as e:
                # surface connection error to caller
                raise
        self.database = database

    def ensure_table(self, table_name: str):
        """Create ClickHouse table for financials if it does not exist.

        Schema is chosen to store normalized records; `metrics` is stored as JSON string.
        Uses ReplacingMergeTree on `_record_key` to allow upserts by replacing rows with same key.
        """
        full = f"{self.database}.{table_name}"
        create_sql = (
            f"CREATE TABLE IF NOT EXISTS {full} ("
            "_record_key String, "
            "symbol String, "
            "company_name String, "
            "report_type String, "
            "period_end Date DEFAULT toDate('1970-01-01'), "
            "report_date Date DEFAULT toDate('1970-01-01'), "
            "fiscal_year UInt16 DEFAULT 0, "
            "fiscal_quarter String, "
            "metrics String, "
            "raw_file_path String, "
            "raw_file_format String, "
            "source_url String, "
            "result_type String, "
            "announcement_date Date DEFAULT toDate('1970-01-01'), "
            "partition_path String"
            ") ENGINE = ReplacingMergeTree() ORDER BY (_record_key)"
        )
        self.client.command(create_sql)

    def _normalize_row(self, rec: Dict) -> Dict:
        """Convert a normalized record into ClickHouse-friendly dict."""
        def _to_date_str(v):
            if v is None:
                return None
            if isinstance(v, str):
                try:
                    # accept ISO date
                    return v[:10]
                except Exception:
                    return None
            if isinstance(v, datetime):
                return v.date().isoformat()
            return None

        row = {
            '_record_key': str(rec.get('_record_key') or ''),
            'symbol': str(rec.get('symbol') or ''),
            'company_name': str(rec.get('company_name') or ''),
            'report_type': str(rec.get('report_type') or ''),
            'period_end': _to_date_str(rec.get('period_end')),
            'report_date': _to_date_str(rec.get('report_date')),
            'fiscal_year': int(rec.get('fiscal_year') or 0),
            'fiscal_quarter': str(rec.get('fiscal_quarter') or ''),
            'metrics': json.dumps(rec.get('metrics') or {}),
            'raw_file_path': str(rec.get('raw_file_path') or rec.get('raw') or ''),
            'raw_file_format': str(rec.get('raw_file_format') or ''),
            'source_url': str(rec.get('source_url') or ''),
            'result_type': str(rec.get('result_type') or ''),
            'announcement_date': _to_date_str(rec.get('announcement_date')),
            'partition_path': str(rec.get('partition_path') or '')
        }
        return row

    def batch_upsert(self, table_name: str, records: Iterable[Dict]):
        """Perform batched upsert of normalized financial records into ClickHouse.

        - Ensures table exists
        - Inserts rows using the client's `insert` method. Uses ReplacingMergeTree to deduplicate on `_record_key`.
        """
        rows = [self._normalize_row(r) for r in records]
        if not rows:
            return 0
        # Ensure table exists
        try:
            self.ensure_table(table_name)
        except Exception as e:
            print(f"Failed to ensure ClickHouse table {table_name}: {e}")
            raise

        # Insert in batches to avoid overloading
        batch_size = 500
        inserted = 0
        for i in range(0, len(rows), batch_size):
            chunk = rows[i:i+batch_size]
            try:
                self.client.insert(f"{self.database}.{table_name}", chunk)
                inserted += len(chunk)
            except Exception as e:
                print(f"ClickHouse insert failed for batch starting at {i}: {e}")
                # continue with next batch
        return inserted


__all__ = ["HudiClickHouseFinancials"]
