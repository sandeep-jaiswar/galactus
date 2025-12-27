"""
Kafka utilities for Galactus platform

Provides Kafka producer and consumer utilities for event-driven data pipeline.
"""

import json
import logging
import time
from typing import Any, Dict, Optional
from datetime import datetime

from kafka import KafkaProducer, KafkaConsumer
from kafka.errors import KafkaError

from conf.config import config


logger = logging.getLogger(__name__)


class NSEKafkaProducer:
    """Kafka producer for NSE data events"""

    def __init__(self):
        # Wait for Kafka to be available
        self._wait_for_kafka()
        
        self.producer = KafkaProducer(
            bootstrap_servers=config.KAFKA_BOOTSTRAP_SERVERS,
            security_protocol=config.KAFKA_SECURITY_PROTOCOL,
            sasl_mechanism=config.KAFKA_SASL_MECHANISM if config.KAFKA_SASL_MECHANISM else None,
            sasl_plain_username=config.KAFKA_SASL_USERNAME if config.KAFKA_SASL_USERNAME else None,
            sasl_plain_password=config.KAFKA_SASL_PASSWORD if config.KAFKA_SASL_PASSWORD else None,
            value_serializer=lambda v: json.dumps(v).encode('utf-8'),
            key_serializer=lambda k: k.encode('utf-8') if k else None,
            retries=3,
            acks='all'
        )
    
    def _wait_for_kafka(self, timeout: int = 60):
        """Wait for Kafka to become available"""
        from kafka import KafkaAdminClient
        from kafka.errors import NoBrokersAvailable
        
        start_time = time.time()
        while time.time() - start_time < timeout:
            try:
                admin_client = KafkaAdminClient(
                    bootstrap_servers=config.KAFKA_BOOTSTRAP_SERVERS,
                    client_id='galactus-producer-check'
                )
                admin_client.close()
                logger.info(f"Successfully connected to Kafka at {config.KAFKA_BOOTSTRAP_SERVERS}")
                return
            except NoBrokersAvailable:
                logger.debug(f"Waiting for Kafka at {config.KAFKA_BOOTSTRAP_SERVERS}...")
                time.sleep(2)
            except Exception as e:
                logger.warning(f"Error checking Kafka connection: {e}")
                time.sleep(2)
        
        raise RuntimeError(f"Timeout waiting for Kafka at {config.KAFKA_BOOTSTRAP_SERVERS}")

    def publish_nse_event(
        self,
        topic: str,
        dataset: str,
        payload: Dict[str, Any],
        symbol: Optional[str] = None,
        event_time: Optional[datetime] = None,
        version: int = 1
    ) -> bool:
        """
        Publish NSE data event to Kafka

        Args:
            topic: Kafka topic name
            dataset: NSE dataset type (bhavcopy, announcements, etc.)
            payload: The actual data payload
            symbol: Optional symbol for partitioning
            event_time: Event timestamp
            version: Data version for handling corrections

        Returns:
            True if successful, False otherwise
        """
        try:
            event = {
                "source": "NSE",
                "dataset": dataset,
                "symbol": symbol,
                "event_time": event_time.isoformat() if event_time else datetime.utcnow().isoformat(),
                "scrape_time": datetime.utcnow().isoformat(),
                "payload": payload,
                "checksum": hash(json.dumps(payload, sort_keys=True)),
                "version": version
            }

            # Use symbol as key for partitioning if available
            key = symbol if symbol else dataset

            future = self.producer.send(topic, value=event, key=key)
            record_metadata = future.get(timeout=10)

            logger.info(f"Published event to {topic} partition {record_metadata.partition} offset {record_metadata.offset}")
            return True

        except KafkaError as e:
            logger.error(f"Failed to publish event to Kafka: {e}")
            return False
        except Exception as e:
            logger.error(f"Unexpected error publishing to Kafka: {e}")
            return False

    def close(self):
        """Close the producer"""
        if self.producer:
            self.producer.close()


class NSEKafkaConsumer:
    """Kafka consumer for NSE data events"""

    def __init__(self, group_id: str, topics: list):
        self.consumer = KafkaConsumer(
            *topics,
            bootstrap_servers=config.KAFKA_BOOTSTRAP_SERVERS,
            security_protocol=config.KAFKA_SECURITY_PROTOCOL,
            sasl_mechanism=config.KAFKA_SASL_MECHANISM if config.KAFKA_SASL_MECHANISM else None,
            sasl_plain_username=config.KAFKA_SASL_USERNAME if config.KAFKA_SASL_USERNAME else None,
            sasl_plain_password=config.KAFKA_SASL_PASSWORD if config.KAFKA_SASL_PASSWORD else None,
            group_id=group_id,
            value_deserializer=lambda v: json.loads(v.decode('utf-8')),
            key_deserializer=lambda k: k.decode('utf-8') if k else None,
            auto_offset_reset='earliest',
            enable_auto_commit=False
        )

    def consume_events(self, timeout_ms: int = 1000):
        """
        Consume events from Kafka topics

        Args:
            timeout_ms: Timeout for polling

        Yields:
            Event dictionaries
        """
        try:
            messages = self.consumer.poll(timeout_ms=timeout_ms)
            for topic_partition, records in messages.items():
                for record in records:
                    yield {
                        'topic': record.topic,
                        'partition': record.partition,
                        'offset': record.offset,
                        'key': record.key,
                        'value': record.value,
                        'timestamp': record.timestamp
                    }
        except KafkaError as e:
            logger.error(f"Error consuming from Kafka: {e}")
            raise

    def commit(self):
        """Manually commit offsets"""
        self.consumer.commit()

    def close(self):
        """Close the consumer"""
        if self.consumer:
            self.consumer.close()


# Global instances
producer = NSEKafkaProducer()
consumer_group = None  # Initialize as needed for specific use cases