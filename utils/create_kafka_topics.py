#!/usr/bin/env python3
"""
Kafka Topic Creation Script

Creates required topics for Galactus NSE data pipeline.
Run this before starting the data ingestion jobs.
"""

import os
import sys
import time
from kafka.admin import KafkaAdminClient, NewTopic
from kafka.errors import TopicAlreadyExistsError, NoBrokersAvailable

# Add project root to path
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from conf.config import config


def create_topics():
    """Create required Kafka topics"""

    topics = [
        NewTopic(
            name="nse.raw.bhavcopy.cash",
            num_partitions=3,
            replication_factor=1,
            topic_configs={
                'retention.ms': str(7 * 24 * 60 * 60 * 1000),  # 7 days
                'segment.ms': str(24 * 60 * 60 * 1000),  # 1 day
            }
        ),
        NewTopic(
            name="nse.raw.announcements",
            num_partitions=3,
            replication_factor=1,
            topic_configs={
                'retention.ms': str(30 * 24 * 60 * 60 * 1000),  # 30 days
                'segment.ms': str(24 * 60 * 60 * 1000),  # 1 day
            }
        ),
        NewTopic(
            name="nse.processed.bhavcopy",
            num_partitions=3,
            replication_factor=1,
            topic_configs={
                'retention.ms': str(30 * 24 * 60 * 60 * 1000),  # 30 days
                'segment.ms': str(24 * 60 * 60 * 1000),  # 1 day
            }
        )
    ]

    try:
        admin_client = KafkaAdminClient(
            bootstrap_servers=config.KAFKA_BOOTSTRAP_SERVERS,
            client_id='galactus-topic-creator'
        )

        print(f"Connecting to Kafka at {config.KAFKA_BOOTSTRAP_SERVERS}...")

        # Create topics
        for topic in topics:
            try:
                admin_client.create_topics([topic])
                print(f"✓ Created topic: {topic.name}")
            except TopicAlreadyExistsError:
                print(f"✓ Topic already exists: {topic.name}")
            except Exception as e:
                print(f"✗ Failed to create topic {topic.name}: {e}")

        admin_client.close()
        print("Topic creation completed.")

    except NoBrokersAvailable:
        print(f"✗ Cannot connect to Kafka at {config.KAFKA_BOOTSTRAP_SERVERS}")
        print("Make sure Kafka is running and accessible.")
        sys.exit(1)
    except Exception as e:
        print(f"✗ Error creating topics: {e}")
        sys.exit(1)


if __name__ == "__main__":
    create_topics()