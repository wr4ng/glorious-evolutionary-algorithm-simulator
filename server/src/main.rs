use axum::{
    routing::{get,post},
    http::StatusCode,
    Json, Router,
	extract::State,
};
use serde::{Deserialize, Serialize};
use std::{collections::{VecDeque} , sync::{Arc, RwLock}};

#[derive()]
struct AppState {
	queue: RwLock<VecDeque<Task>>,
	finished: RwLock<Vec<Result>>
}

type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() {
	// tracing_subscriber::fmt::init();
	let state: SharedState = Arc::new(AppState{
		queue: RwLock::new(VecDeque::new()),
		finished: RwLock::new(Vec::new())
	});

    // build our application with a single route
    let app = Router::new()
    .route("/", get(root))
	.route("/tasks", 
		get(get_tasks)
		.post(create_task))
    .route("/tasks/ID", 
		get(get_task))
	.with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_task(Json(id): Json<ID>) -> (StatusCode) {
	//TODO Find Task with ID, skal heller ikke retunere status code men en json
	StatusCode::ACCEPTED
}


async fn create_task(State(state): State<SharedState>, Json(request): Json<CreateTaskRequest>) -> (StatusCode) {	
	let task = Task {
		id: 1, //TODO random or counting
		algorithm: request.algorithm,
		problem: request.problem,
		stop_cond: request.stop_cond,
		task_param: request.task_param,
		parameters: request.parameters,
		body: request.body,
	};
	
	state.queue.write().unwrap().push_back(task);

	StatusCode::CREATED
}

async fn get_tasks(State(state): State<SharedState>) -> Json<Vec<Task>> {
	let tasks = state.queue.read().unwrap().clone().into_iter().collect();
	Json(tasks)
}

#[derive(Deserialize)]
struct ID {
	id: usize,
}
#[derive(Deserialize)]
struct CreateTaskRequest {
	algorithm: Algorithm,
	problem: Problem,
	stop_cond: StopCondition,
	task_param: TaskParameters,
	parameters: AlgoParameters,
	body: String,
}

#[derive(Serialize, Clone)]
struct Task {
	id:  u64,
	algorithm: Algorithm,
	problem: Problem,
	stop_cond: StopCondition,
	task_param: TaskParameters,
	parameters: AlgoParameters,
	body: String,
}

#[derive(Deserialize, Serialize, Clone)]
enum Algorithm {
	OnePlusOneEA,
	SimulatedAnnealing,
	ACO,
}
#[derive(Deserialize, Serialize, Clone)]
enum Problem {
	OneMax,
	TSP,
	LeadginOnes,
}

#[derive(Deserialize, Serialize, Clone)]
struct StopCondition {
	max_time: Option<usize>,
	max_iterations: Option<usize>,
	max_iterations_since_improvement: Option<usize>,
	requested_fitness: Option<usize>,
}

#[derive(Deserialize, Serialize, Clone)]
struct TaskParameters {
	repetitions: Option<usize>,
	data_interval: Option<usize>,
	measure_time: Option<bool>,
	save_running_solutions: Option<bool>,
	save_running_fitness: Option<bool>
}

#[derive(Deserialize, Serialize, Clone)]
struct AlgoParameters {
	//TODO
}

struct Result {
	id: usize,
	algorithm: Algorithm,
	problem: Problem,	
}