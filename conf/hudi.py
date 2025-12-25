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

        # Disable Java-based meta sync (use Python sync module instead)
        "hoodie.meta.sync.enable": "false",
        "hoodie.datasource.meta.sync.enable": "false",
        "hoodie.meta.sync.classes": "",
        "hoodie.datasource.hive_sync.enable": "false",
        "hoodie.datasource.clickhouse_sync.enable": "false",
        "hoodie.datasource.clickhouse_sync.jdbc.url": "",
        "hoodie.datasource.clickhouse_sync.table": "",
        "hoodie.datasource.clickhouse_sync.database": "",
        "hoodie.datasource.clickhouse_sync.username": "",
        "hoodie.datasource.clickhouse_sync.password": "",
        "hoodie.datasource.clickhouse_sync.partition_fields": partition_key,
        "hoodie.datasource.clickhouse_sync.partition_extractor_class": "org.apache.hudi.hive.SlashEncodedDayPartitionValueExtractor",
        # Hive sync (HMS) settings left empty when disabled
        "hoodie.datasource.hive_sync.mode": "NONE",
        "hoodie.datasource.hive_sync.metastore.uris": "",
        "hoodie.datasource.hive_sync.database": "",
        "hoodie.datasource.hive_sync.table": "",
        "hoodie.datasource.hive_sync.partition_extractor_class": ""
    }
