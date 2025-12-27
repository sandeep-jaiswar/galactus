//! Real-time Event Streaming
//!
//! Provides streaming infrastructure for real-time event processing
//! with connection resilience and event emission.

use crate::data::CanonicalEvent;
use crate::ingestion::sources::{DataSource, ConnectionState, SourceError};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Event stream errors
#[derive(Debug, Clone)]
pub enum StreamError {
    /// Source error
    SourceError(String),
    /// Channel error
    ChannelError(String),
    /// Processing error
    ProcessingError(String),
}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::SourceError(msg) => write!(f, "Source error: {}", msg),
            StreamError::ChannelError(msg) => write!(f, "Channel error: {}", msg),
            StreamError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
        }
    }
}

impl std::error::Error for StreamError {}

impl From<SourceError> for StreamError {
    fn from(err: SourceError) -> Self {
        StreamError::SourceError(err.to_string())
    }
}

/// Event handler trait
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle an incoming event
    async fn handle_event(&self, event: CanonicalEvent) -> Result<(), StreamError>;
}

/// Event stream processor
pub struct EventStream {
    event_rx: mpsc::Receiver<CanonicalEvent>,
    handlers: Vec<Arc<dyn EventHandler>>,
    is_running: Arc<RwLock<bool>>,
}

impl EventStream {
    /// Create a new event stream
    pub fn new(event_rx: mpsc::Receiver<CanonicalEvent>) -> Self {
        Self {
            event_rx,
            handlers: Vec::new(),
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    /// Add an event handler
    pub fn add_handler(&mut self, handler: Arc<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    /// Start processing events
    pub async fn start(&mut self) -> Result<(), StreamError> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            return Err(StreamError::ProcessingError("Stream already running".to_string()));
        }
        *is_running = true;
        drop(is_running);

        while let Some(event) = self.event_rx.recv().await {
            // Process event with all handlers
            for handler in &self.handlers {
                if let Err(e) = handler.handle_event(event.clone()).await {
                    eprintln!("Handler error: {}", e);
                    // Continue processing with other handlers
                }
            }

            // Check if we should stop
            let running = self.is_running.read().await;
            if !*running {
                break;
            }
        }

        Ok(())
    }

    /// Stop processing events
    pub async fn stop(&self) {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
    }

    /// Check if stream is running
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }
}

/// Resilient stream manager with automatic reconnection
pub struct ResilientStream {
    source: Box<dyn DataSource>,
    reconnect_delay: Duration,
    max_reconnect_attempts: usize,
    heartbeat_interval: Duration,
}

impl ResilientStream {
    /// Create a new resilient stream
    pub fn new(
        source: Box<dyn DataSource>,
        reconnect_delay: Duration,
        max_reconnect_attempts: usize,
        heartbeat_interval: Duration,
    ) -> Self {
        Self {
            source,
            reconnect_delay,
            max_reconnect_attempts,
            heartbeat_interval,
        }
    }

    /// Start the stream with automatic reconnection
    pub async fn start(&mut self) -> Result<(), StreamError> {
        let mut attempts = 0;

        loop {
            // Try to connect
            match self.source.connect().await {
                Ok(_) => {
                    println!("Connected successfully");
                    attempts = 0; // Reset on successful connection
                    
                    // Monitor connection health
                    self.monitor_connection().await;
                    
                    // If we get here, connection was lost
                    println!("Connection lost, attempting reconnect...");
                }
                Err(e) => {
                    attempts += 1;
                    println!("Connection failed (attempt {}): {}", attempts, e);
                    
                    if attempts >= self.max_reconnect_attempts {
                        return Err(StreamError::SourceError(
                            format!("Max reconnection attempts ({}) exceeded", self.max_reconnect_attempts)
                        ));
                    }
                }
            }

            // Wait before reconnecting
            tokio::time::sleep(self.reconnect_delay).await;
        }
    }

    /// Monitor connection health with heartbeat
    async fn monitor_connection(&mut self) {
        let mut heartbeat = interval(self.heartbeat_interval);

        loop {
            heartbeat.tick().await;

            // Check connection state
            match self.source.state() {
                ConnectionState::Connected => {
                    // Connection is healthy, continue
                }
                _ => {
                    // Connection lost or in bad state
                    println!("Connection health check failed");
                    let _ = self.source.disconnect().await;
                    break;
                }
            }
        }
    }
}

/// Simple event logger handler for testing
pub struct EventLogger;

#[async_trait::async_trait]
impl EventHandler for EventLogger {
    async fn handle_event(&self, event: CanonicalEvent) -> Result<(), StreamError> {
        println!("Received event: {:?}", event.event_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::*;
    use crate::ingestion::sources::WebSocketSource;
    use crate::config::StreamingConfig;
    use chrono::Utc;

    fn create_test_config() -> StreamingConfig {
        StreamingConfig {
            nse_websocket_url: "wss://test.com/nse".to_string(),
            bse_websocket_url: "wss://test.com/bse".to_string(),
            reconnect_delay_seconds: 1,
            max_reconnect_attempts: 3,
            heartbeat_interval_seconds: 30,
            auto_reconnect: true,
        }
    }

    #[tokio::test]
    async fn test_event_stream_creation() {
        let (_tx, rx) = mpsc::channel(10);
        let stream = EventStream::new(rx);
        assert!(!stream.is_running().await);
    }

    #[tokio::test]
    async fn test_event_logger() {
        let event = CanonicalEvent::new(
            EventId("test-1".to_string()),
            EventType::Liquidity,
            Utc::now(),
            DataSource("TEST".to_string()),
            vec![Instrument("NIFTY".to_string())],
            EventPayload::Liquidity(LiquidityPayload {
                instrument: Instrument("NIFTY".to_string()),
                traded_volume: 1000,
                delivery_volume: 800,
                delivery_ratio: 0.8,
                avg_daily_value: 10000.0,
            }),
            Completeness::Complete,
            SchemaVersion("1.0".to_string()),
        );

        let logger = EventLogger;
        let result = logger.handle_event(event).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_resilient_stream_creation() {
        let config = create_test_config();
        let source = Box::new(WebSocketSource::new("wss://test.com".to_string(), config.clone()));
        
        let stream = ResilientStream::new(
            source,
            Duration::from_secs(config.reconnect_delay_seconds),
            config.max_reconnect_attempts,
            Duration::from_secs(config.heartbeat_interval_seconds),
        );

        // Just test creation, not actual running
        assert!(true);
    }
}
