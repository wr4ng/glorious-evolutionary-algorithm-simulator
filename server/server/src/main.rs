use axum::{
    Router,
    routing::{get, post},
};
use schedule::{TaskSchedule, create_task_schedule};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;
use ws::handle_websocket_connect;

mod create;
mod schedule;
mod ws;

#[derive()]
struct AppState {
    pending_schedules: HashMap<Uuid, TaskSchedule>,
}

type SharedState = Arc<Mutex<AppState>>;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(AppState {
        pending_schedules: HashMap::new(),
    }));

    // Permissive CORS layer, allowing web-client to fetch from another origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/ping", get(ping_handler))
        .route("/schedules", post(create_task_schedule))
        .route("/ws/{id}", get(handle_websocket_connect))
        .layer(cors)
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    println!("listining at 0.0.0.0:3000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Simple ping-pong endpoint used to verify is server is running at selected address
async fn ping_handler() -> String {
    "pong".to_owned()
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Task {
    algorithm: AlgorithmConfig,
    problem: Problem,
    stop_cond: StopCondition,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct StopCondition {
    max_iterations: u64,
    optimal_fitness: Option<f64>,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
#[serde(tag = "type")]
enum AlgorithmConfig {
    OnePlusOneEA,
    SimulatedAnnealing {
        cooling_schedule: CoolingSchedule,
    },
    ACO {
        alpha: f64,
        beta: f64,
        evap_factor: f64,
        ants: usize,
        p_best: Option<f64>,
        q: Option<f64>,
        nn: bool,
        update_strategy: UpdateStrategy,
    },
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
#[serde(tag = "type")]
enum CoolingSchedule {
    Static { temperature: f64 },
    Exponential { cooling_rate: f64 },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
enum Problem {
    OneMax {
        bitstring_size: usize,
    },
    LeadingOnes {
        bitstring_size: usize,
    },
    TSP {
        tsp_instance: String,
        tsp_name: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
enum UpdateStrategy {
    BestSoFar,
    GenerationBest,
    AllAnts,
}
