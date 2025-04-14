use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use create::{CreateError, create_ea};
use rand::rng;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};
use tokio::sync::broadcast::{Sender, channel};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;
use ws::get_websocket_updates;

mod create;
mod ws;

#[derive()]
struct AppState {
    in_progress: HashMap<Uuid, Task>,
    in_progress_channels: HashMap<Uuid, Sender<serde_json::Value>>,
    queue: VecDeque<Task>,
    finished: Vec<TaskResult>,
}

type SharedState = Arc<Mutex<AppState>>;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(AppState {
        in_progress: HashMap::new(),
        in_progress_channels: HashMap::new(),
        queue: VecDeque::new(),
        finished: Vec::new(),
    }));

    //TODO: Proberly handle CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/ping", get(ping_handler))
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/ws/{id}", get(get_websocket_updates))
        .layer(cors)
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    println!("listining at 0.0.0.0:3000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ping_handler() -> String {
    "pong".to_owned()
}

async fn create_task(
    State(state): State<SharedState>,
    Json(request): Json<CreateTaskRequest>,
) -> Result<Json<Task>, CreateError> {
    let id = Uuid::new_v4();
    let task = Task {
        id,
        algorithm: request.algorithm,
        problem: request.problem,
        tsp_instance: request.tsp_instance.clone(),
        stop_cond: request.stop_cond.clone(),
    };

    let mut runner = create_ea(request.clone())?;

    //TODO: Push to instead if there are no threads avaiable to pick up task
    state
        .lock()
        .expect("failed to aquire mutex")
        .in_progress
        .insert(id, task.clone());

    thread::spawn(move || {
        println!("initial: {}", runner.current_fitness());

        //Insert channel
        let (tx, _) = channel::<serde_json::Value>(10); //TODO: Determine capacity
        state
            .lock()
            .expect("failed to aquire mutex")
            .in_progress_channels
            .insert(task.id, tx.clone());

        sleep(Duration::from_millis(100));

        let _ = tx.send(runner.status_json());
        loop {
            runner.iterate(&mut rng());
            if runner.iterations() >= task.stop_cond.max_iterations {
                break;
            }
            // If optimal solution is provided, stop if it is reached
            if let Some(optimal) = task.stop_cond.optimal_fitness {
                if optimal == runner.current_fitness() {
                    break;
                }
            }
            //TODO: Don't use fixed update-rate
            if runner.iterations() % 1000 == 0 {
                let _ = tx.send(runner.status_json());
            }
        }

        let _ = tx.send(runner.status_json());
        println!("result: {}", runner.current_fitness());

        // Keep lock on shared state while removing from in_progress and inserting into finished
        {
            let mut state = state.lock().expect("failed to aquire mutex");
            state.in_progress.remove(&task.id);
            state.in_progress_channels.remove(&task.id);
            state.finished.push(TaskResult {
                id,
                algorithm: request.algorithm,
                problem: request.problem,
                final_fitness: runner.current_fitness(),
            });
        }
    });

    Ok(Json(task))
}

async fn get_tasks(State(state): State<SharedState>) -> (StatusCode, Json<TasksReturn>) {
    let state = state.lock().expect("couldn't aquire mutex");
    let in_progress = state.in_progress.clone().into_values().collect();
    let queued = state.queue.clone().into_iter().collect();
    let finished = state.finished.clone();
    let returns = TasksReturn {
        in_progress,
        queued,
        finished,
    };
    (StatusCode::ACCEPTED, Json(returns))
}

#[derive(Deserialize, Clone)]
struct CreateTaskRequest {
    algorithm: Algorithm,
    problem: Problem,
    bitstring_size: Option<u32>,
    tsp_instance: Option<String>,
    stop_cond: StopCondition,
}

#[derive(Serialize, Clone)]
struct Task {
    id: Uuid,
    algorithm: Algorithm,
    problem: Problem,
    tsp_instance: Option<String>,
    stop_cond: StopCondition,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
enum Algorithm {
    OnePlusOneEA,
    SimulatedAnnealing,
    ACO,
}
#[derive(Deserialize, Serialize, Clone, Copy)]
enum Problem {
    OneMax,
    LeadingOnes,
    TSP,
}

#[derive(Deserialize, Serialize, Clone)]
struct StopCondition {
    max_iterations: u64,
    optimal_fitness: Option<f64>,
}

#[derive(Serialize, Clone)]
struct TaskResult {
    id: Uuid,
    algorithm: Algorithm,
    problem: Problem,
    final_fitness: f64,
}

#[derive(Serialize)]
struct TasksReturn {
    in_progress: Vec<Task>,
    queued: Vec<Task>,
    finished: Vec<TaskResult>,
}
