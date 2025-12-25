# conf/hudi.py

def hudi_write_options(
    table_name: str,
    record_key: str,
    precombine_key: str,
    partition_key: str
) -> dict:
    """
    Standard Hudi UPSERT config for post-trade datasets
    """

    return {
        # Core table config
        "hoodie.table.name": table_name,
        "hoodie.datasource.write.table.type": "COPY_ON_WRITE",
        "hoodie.datasource.write.operation": "upsert",

        # Keys
        "hoodie.datasource.write.recordkey.field": record_key,
        "hoodie.datasource.write.precombine.field": precombine_key,
        "hoodie.datasource.write.partitionpath.field": partition_key,

        # Partitioning
        "hoodie.datasource.write.hive_style_partitioning": "true",

        # Performance / safety
        "hoodie.datasource.write.insert.drop.duplicates": "true",
        "hoodie.clean.automatic": "true",
        "hoodie.clean.async": "true",

        # Timeline & metadata
        "hoodie.metadata.enable": "true",

        # Schema evolution (important for finance)
        "hoodie.datasource.write.schema.evolution.enable": "true",

        # ClickHouse sync configuration
        "hoodie.meta.sync.enable": "true",
        "hoodie.datasource.meta.sync.enable": "true",
        "hoodie.datasource.hive_sync.enable": "false",  # Disable Hive sync
        "hoodie.datasource.clickhouse_sync.enable": "true",
        "hoodie.datasource.clickhouse_sync.jdbc.url": "jdbc:clickhouse://localhost:8123/galactus",
        "hoodie.datasource.clickhouse_sync.table": table_name,
        "hoodie.datasource.clickhouse_sync.database": "galactus",
        "hoodie.datasource.clickhouse_sync.username": "",
        "hoodie.datasource.clickhouse_sync.password": "",
        "hoodie.datasource.clickhouse_sync.partition_fields": partition_key,
        "hoodie.datasource.clickhouse_sync.partition_extractor_class": "org.apache.hudi.hive.SlashEncodedDayPartitionValueExtractor"
    }
