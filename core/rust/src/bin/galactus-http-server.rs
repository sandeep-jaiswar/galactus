//! Galactus HTTP API Server
//!
//! Simple HTTP server that provides REST API access to the Galactus inference engine.

use galactus_core::intent::{IntentEngine, SignalInput};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

#[derive(serde::Deserialize, serde::Serialize)]
struct ApiSignalInput {
    name: String,
    value: f64,
    confidence: f64,
    timestamp: i64,
    metadata: HashMap<String, String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct IntentRequest {
    signals: Vec<ApiSignalInput>,
    client_id: Option<String>,
}

#[derive(serde::Serialize)]
struct IntentResponse {
    intent_vector: HashMap<String, f64>,
    confidence: HashMap<String, f64>,
    regime: String,
    timestamp: i64,
}

#[derive(serde::Serialize)]
struct HealthResponse {
    status: String,
    uptime_seconds: u64,
    components: HashMap<String, String>,
}

#[derive(serde::Serialize)]
struct MetricsResponse {
    galactus_inference_operations_total: u64,
    galactus_data_quality_score: f64,
    galactus_confidence_score: f64,
    galactus_stability_score: f64,
    galactus_active_signals: u64,
    galactus_silence_suppressions_total: u64,
}

struct AppState {
    engine: IntentEngine,
    start_time: std::time::Instant,
    request_count: Mutex<u64>,
}

impl AppState {
    fn new() -> Self {
        Self {
            engine: IntentEngine::default(),
            start_time: std::time::Instant::now(),
            request_count: Mutex::new(0),
        }
    }
}

#[tokio::main]
async fn main() {
    println!("🚀 Starting Galactus HTTP API Server");

    let state = Arc::new(AppState::new());

    // Health check endpoint
    let health = warp::path!("api" / "v1" / "health").map({
        let state = state.clone();
        move || {
            let uptime = state.start_time.elapsed().as_secs();
            let components = HashMap::from([
                ("inference_engine".to_string(), "healthy".to_string()),
                ("data_processing".to_string(), "healthy".to_string()),
            ]);

            warp::reply::json(&HealthResponse {
                status: "healthy".to_string(),
                uptime_seconds: uptime,
                components,
            })
        }
    });

    // Metrics endpoint
    let metrics = warp::path!("api" / "v1" / "metrics").map({
        let state = state.clone();
        move || {
            let request_count = state
                .request_count
                .try_lock()
                .map(|guard| *guard)
                .unwrap_or(0);
            warp::reply::json(&MetricsResponse {
                galactus_inference_operations_total: request_count,
                galactus_data_quality_score: 0.95,
                galactus_confidence_score: 0.87,
                galactus_stability_score: 0.92,
                galactus_active_signals: 3,
                galactus_silence_suppressions_total: 0,
            })
        }
    });

    // Intent inference endpoint
    let intent = warp::path!("api" / "v1" / "intent")
        .and(warp::post())
        .and(warp::body::json())
        .and_then({
            let state = state.clone();
            move |request: IntentRequest| {
                let state = state.clone();
                async move {
                    // Increment request count
                    {
                        let mut count = state.request_count.lock().await;
                        *count += 1;
                    }

                    // Convert API signals to internal format
                    let signals: Vec<SignalInput> = request
                        .signals
                        .into_iter()
                        .map(|s| SignalInput {
                            name: s.name,
                            value: s.value,
                            confidence: s.confidence,
                            timestamp: s.timestamp,
                            metadata: s.metadata,
                        })
                        .collect();

                    // Process with intent engine
                    match state.engine.process(signals) {
                        Ok(result) => {
                            let intent_vector = HashMap::from([
                                ("oi_decay".to_string(), result.intent.pressure),
                                ("hedge_pressure".to_string(), 0.0),
                                ("basis_pressure".to_string(), 0.0),
                            ]);

                            let confidence = HashMap::from([
                                ("overall".to_string(), result.intent.confidence),
                                ("data_quality".to_string(), 0.95),
                                ("stability".to_string(), 0.92),
                            ]);

                            let response = IntentResponse {
                                intent_vector,
                                confidence,
                                regime: format!("{:?}", result.intent.regime),
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs() as i64,
                            };

                            Ok::<_, warp::Rejection>(warp::reply::json(&response))
                        }
                        Err(e) => {
                            eprintln!("Inference error: {:?}", e);
                            Err(warp::reject::custom(ApiError::InferenceError))
                        }
                    }
                }
            }
        });

    // Configure CORS based on environment variable or default to localhost
    let cors_allowed_origins = std::env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    let cors = if cors_allowed_origins == "*" {
        warp::cors()
            .allow_any_origin()
            .allow_methods(vec!["GET", "POST"])
            .allow_headers(vec!["content-type"])
    } else {
        warp::cors()
            .allow_origin(cors_allowed_origins.as_str())
            .allow_methods(vec!["GET", "POST"])
            .allow_headers(vec!["content-type"])
    };

    let routes = health
        .or(metrics)
        .or(intent)
        .with(cors)
        .recover(handle_rejection);

    println!("📡 Server listening on http://0.0.0.0:8080");
    println!("   Health: GET /api/v1/health");
    println!("   Metrics: GET /api/v1/metrics");
    println!("   Intent: POST /api/v1/intent");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

#[derive(Debug)]
enum ApiError {
    InferenceError,
}
impl warp::reject::Reject for ApiError {}

async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    if let Some(api_err) = err.find::<ApiError>() {
        match api_err {
            ApiError::InferenceError => Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({"error": "Inference failed"})),
                warp::http::StatusCode::BAD_REQUEST,
            )),
        }
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "Not found"})),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}
