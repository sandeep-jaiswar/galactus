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
            df = self.spark.read.format("hudi") \
                .load(base_path) \
                .select(
                    "symbol", "series", "open_price", "high_price", "low_price",
                    "close_price", "last_price", "prev_close", "total_traded_qty",
                    "turnover_lacs", "DATE1", "no_of_trades", "record_key",
                    "trade_date", "_hoodie_commit_time", "_hoodie_commit_seqno",
                    "_hoodie_record_key", "_hoodie_partition_path", "_hoodie_file_name"
                )
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
            # Convert to pandas for ClickHouse insertion
            pdf = df.toPandas()

            # Prepare data for ClickHouse - we'll use UPSERT logic
            data = []
            
            for _, row in pdf.iterrows():
                # Convert DATE1 string to datetime
                date_str = str(row['DATE1']) if row['DATE1'] else None
                timestamp = None
                if date_str:
                    try:
                        # Parse date string like "23-DEC-2025" 
                        timestamp = datetime.strptime(date_str, "%d-%b-%Y")
                    except ValueError:
                        # If parsing fails, use current time
                        timestamp = datetime.now()
                
                # Convert trade_date string to date
                trade_date_str = str(row['trade_date']) if row['trade_date'] else None
                trade_date = None
                if trade_date_str:
                    try:
                        # Parse date string like "2025-12-23"
                        trade_date = datetime.strptime(trade_date_str, "%Y-%m-%d").date()
                    except ValueError:
                        # If parsing fails, use current date
                        trade_date = datetime.now().date()
                
                data.append([
                    str(row['symbol']) if row['symbol'] else '',
                    str(row['series']) if row['series'] else '',
                    float(row['open_price']) if row['open_price'] else 0.0,
                    float(row['high_price']) if row['high_price'] else 0.0,
                    float(row['low_price']) if row['low_price'] else 0.0,
                    float(row['close_price']) if row['close_price'] else 0.0,
                    float(row['last_price']) if row['last_price'] else 0.0,
                    float(row['prev_close']) if row['prev_close'] else 0.0,
                    int(row['total_traded_qty']) if row['total_traded_qty'] else 0,
                    float(row['turnover_lacs']) if row['turnover_lacs'] else 0.0,
                    timestamp,  # Use parsed datetime object
                    int(row['no_of_trades']) if row['no_of_trades'] else 0,
                    str(row['record_key']) if row['record_key'] else '',
                    trade_date,  # Use parsed date object
                    str(row['_hoodie_commit_time']) if row['_hoodie_commit_time'] else '',
                    str(row['_hoodie_commit_seqno']) if row['_hoodie_commit_seqno'] else '',
                    str(row['_hoodie_record_key']) if row['_hoodie_record_key'] else '',
                    str(row['_hoodie_partition_path']) if row['_hoodie_partition_path'] else '',
                    str(row['_hoodie_file_name']) if row['_hoodie_file_name'] else ''
                ])

            if not data:
                logger.info("No records to process")
                return True

            # Use ClickHouse's INSERT with ON CONFLICT for upsert
            # First, delete existing records with same symbol + trade_date combination
            symbols_and_dates = [(row[0], row[13]) for row in data if row[0] and row[13]]
            
            if symbols_and_dates:
                # Delete existing records to prepare for upsert
                delete_conditions = []
                for symbol, trade_date in symbols_and_dates:
                    delete_conditions.append(f"(symbol = '{symbol}' AND trade_date = '{trade_date}')")
                
                if delete_conditions:
                    delete_query = f"DELETE FROM {table_name} WHERE {' OR '.join(delete_conditions)}"
                    try:
                        self.ch_client.command(delete_query)
                        logger.info(f"Deleted existing records for {len(symbols_and_dates)} symbol+trade_date combinations")
                    except Exception as e:
                        logger.warning(f"Could not delete existing records: {e}. Proceeding with insert.")

            # Insert all data (this will be the "upsert" since we deleted existing records)
            self.ch_client.insert(table_name, data,
                                column_names=[
                                    'symbol', 'series', 'open_price', 'high_price', 'low_price',
                                    'close_price', 'last_price', 'prev_close', 'total_traded_qty',
                                    'total_traded_val', 'timestamp', 'total_trades', 'isin',
                                    'trade_date', '_hoodie_commit_time', '_hoodie_commit_seqno',
                                    '_hoodie_record_key', '_hoodie_partition_path', '_hoodie_file_name'
                                ])

            logger.info(f"Successfully upserted {len(data)} records to ClickHouse table {table_name}")
            return True

        except Exception as e:
            logger.error(f"Error syncing to ClickHouse: {e}")
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