//! Real-time Data Source Connections
//!
//! Provides WebSocket connections to various data sources (exchanges)
//! with automatic reconnection and error handling.

use crate::data::CanonicalEvent;
use crate::config::StreamingConfig;
use tokio::sync::mpsc;

/// Data source connection errors
#[derive(Debug, Clone, PartialEq)]
pub enum SourceError {
    /// Connection failed
    ConnectionFailed(String),
    /// Authentication failed
    AuthenticationFailed(String),
    /// Protocol error
    ProtocolError(String),
    /// Timeout
    Timeout,
    /// Connection closed
    Closed,
}

impl std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            SourceError::AuthenticationFailed(msg) => write!(f, "Authentication failed: {}", msg),
            SourceError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            SourceError::Timeout => write!(f, "Connection timeout"),
            SourceError::Closed => write!(f, "Connection closed"),
        }
    }
}

impl std::error::Error for SourceError {}

/// Connection state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected
    Disconnected,
    /// Connecting
    Connecting,
    /// Connected and active
    Connected,
    /// Reconnecting after failure
    Reconnecting,
    /// Failed permanently
    Failed,
}

/// Data source trait for different exchange connections
#[async_trait::async_trait]
pub trait DataSource: Send + Sync {
    /// Connect to the data source
    async fn connect(&mut self) -> Result<(), SourceError>;
    
    /// Disconnect from the data source
    async fn disconnect(&mut self) -> Result<(), SourceError>;
    
    /// Get current connection state
    fn state(&self) -> ConnectionState;
    
    /// Subscribe to specific instruments
    async fn subscribe(&mut self, instruments: Vec<String>) -> Result<(), SourceError>;
    
    /// Unsubscribe from instruments
    async fn unsubscribe(&mut self, instruments: Vec<String>) -> Result<(), SourceError>;
    
    /// Get the event receiver channel
    fn event_receiver(&self) -> mpsc::Receiver<CanonicalEvent>;
}

/// WebSocket data source implementation
#[allow(dead_code)]
pub struct WebSocketSource {
    config: StreamingConfig,
    endpoint: String,
    state: ConnectionState,
    event_tx: mpsc::Sender<CanonicalEvent>,
    event_rx: Option<mpsc::Receiver<CanonicalEvent>>,
    subscriptions: Vec<String>,
    reconnect_attempts: usize,
}

impl WebSocketSource {
    /// Create a new WebSocket data source
    pub fn new(endpoint: String, config: StreamingConfig) -> Self {
        let (event_tx, event_rx) = mpsc::channel(1000);
        
        Self {
            config,
            endpoint,
            state: ConnectionState::Disconnected,
            event_tx,
            event_rx: Some(event_rx),
            subscriptions: Vec::new(),
            reconnect_attempts: 0,
        }
    }

    /// Check if reconnection should be attempted
    #[allow(dead_code)]
    fn should_reconnect(&self) -> bool {
        self.config.auto_reconnect && self.reconnect_attempts < self.config.max_reconnect_attempts
    }

    /// Reset reconnection counter
    #[allow(dead_code)]
    fn reset_reconnect_counter(&mut self) {
        self.reconnect_attempts = 0;
    }

    /// Increment reconnection counter
    #[allow(dead_code)]
    fn increment_reconnect_counter(&mut self) {
        self.reconnect_attempts += 1;
    }
}

#[async_trait::async_trait]
impl DataSource for WebSocketSource {
    async fn connect(&mut self) -> Result<(), SourceError> {
        self.state = ConnectionState::Connecting;
        
        // TODO: Implement actual WebSocket connection
        // For now, simulate connection
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        self.state = ConnectionState::Connected;
        self.reset_reconnect_counter();
        
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), SourceError> {
        // TODO: Implement actual disconnection
        self.state = ConnectionState::Disconnected;
        Ok(())
    }

    fn state(&self) -> ConnectionState {
        self.state.clone()
    }

    async fn subscribe(&mut self, instruments: Vec<String>) -> Result<(), SourceError> {
        if self.state != ConnectionState::Connected {
            return Err(SourceError::ConnectionFailed("Not connected".to_string()));
        }

        // Add to subscription list
        for instrument in instruments {
            if !self.subscriptions.contains(&instrument) {
                self.subscriptions.push(instrument);
            }
        }

        // TODO: Send subscription message over WebSocket
        
        Ok(())
    }

    async fn unsubscribe(&mut self, instruments: Vec<String>) -> Result<(), SourceError> {
        if self.state != ConnectionState::Connected {
            return Err(SourceError::ConnectionFailed("Not connected".to_string()));
        }

        // Remove from subscription list
        self.subscriptions.retain(|s| !instruments.contains(s));

        // TODO: Send unsubscription message over WebSocket
        
        Ok(())
    }

    fn event_receiver(&self) -> mpsc::Receiver<CanonicalEvent> {
        // This is a simplified implementation
        // In production, we would properly handle the receiver
        let (_tx, rx) = mpsc::channel(1000);
        rx
    }
}

/// Data source manager for multiple connections
#[allow(dead_code)]
pub struct SourceManager {
    sources: Vec<Box<dyn DataSource>>,
    config: StreamingConfig,
}

impl SourceManager {
    /// Create a new source manager
    pub fn new(config: StreamingConfig) -> Self {
        Self {
            sources: Vec::new(),
            config,
        }
    }

    /// Add a data source
    pub fn add_source(&mut self, source: Box<dyn DataSource>) {
        self.sources.push(source);
    }

    /// Connect all sources
    pub async fn connect_all(&mut self) -> Result<(), SourceError> {
        for source in &mut self.sources {
            source.connect().await?;
        }
        Ok(())
    }

    /// Disconnect all sources
    pub async fn disconnect_all(&mut self) -> Result<(), SourceError> {
        for source in &mut self.sources {
            source.disconnect().await?;
        }
        Ok(())
    }

    /// Get connection states
    pub fn get_states(&self) -> Vec<ConnectionState> {
        self.sources.iter().map(|s| s.state()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    async fn test_websocket_source_creation() {
        let config = create_test_config();
        let source = WebSocketSource::new("wss://test.com".to_string(), config);
        assert_eq!(source.state(), ConnectionState::Disconnected);
    }

    #[tokio::test]
    async fn test_websocket_connect() {
        let config = create_test_config();
        let mut source = WebSocketSource::new("wss://test.com".to_string(), config);
        
        let result = source.connect().await;
        assert!(result.is_ok());
        assert_eq!(source.state(), ConnectionState::Connected);
    }

    #[tokio::test]
    async fn test_subscribe_before_connect() {
        let config = create_test_config();
        let mut source = WebSocketSource::new("wss://test.com".to_string(), config);
        
        let result = source.subscribe(vec!["NIFTY".to_string()]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_source_manager() {
        let config = create_test_config();
        let mut manager = SourceManager::new(config.clone());
        
        let source = Box::new(WebSocketSource::new("wss://test.com".to_string(), config));
        manager.add_source(source);
        
        assert_eq!(manager.sources.len(), 1);
    }
}
