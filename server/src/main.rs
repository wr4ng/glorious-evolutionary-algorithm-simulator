use axum::{
    routing::{get,post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() {
	// tracing_subscriber::fmt::init();

    // build our application with a single route
    let app = Router::new()
    .route("/", get(root))
	.route("/tasks", 
		//get(get_tasks).
		post(create_task))
    .route("/tasks/ID", 
		get(get_task));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_task(Json(id): Json<ID>) -> (StatusCode) {
	//TODO Find Task with ID, skal heller ikke retunere status code men en json
	(StatusCode::CREATED)
}


async fn create_task(Json(request): Json<CreateTaskRequest>) -> (StatusCode, Json<Task>) {
	let task = Task {
		id: 1, //TODO random or counting
		algorithm: request.algorithm,
		problem: request.problem,
		stop_cond: request.stop_cond,
		task_param: request.task_param,
		parameters: request.parameters,
		body: request.body,
	};
	
	(StatusCode::CREATED, Json(task))
}
#[derive(Deserialize)]
struct ID {
	id: usize,
}
//TODO
// async fn get_tasks() -> Json<Vec<Task>> {
	
// }

#[derive(Deserialize)]
struct CreateTaskRequest {
	algorithm: Algorithm,
	problem: Problem,
	stop_cond: StopCondition,
	task_param: TaskParameters,
	parameters: AlgoParameters,
	body: String,
}

#[derive(Serialize)]
struct Task {
	id:  u64,
	algorithm: Algorithm,
	problem: Problem,
	stop_cond: StopCondition,
	task_param: TaskParameters,
	parameters: AlgoParameters,
	body: String,
}

#[derive(Deserialize, Serialize)]
enum Algorithm {
	OnePlusOneEA,
	SimulatedAnnealing,
	ACO,
}
#[derive(Deserialize, Serialize)]
enum Problem {
	OneMax,
	TSP,
	LeadginOnes,
}

#[derive(Deserialize, Serialize)]
struct StopCondition {
	max_time: Option<usize>,
	max_iterations: Option<usize>,
	max_iterations_since_improvement: Option<usize>,
	requested_fitness: Option<usize>,
}

#[derive(Deserialize, Serialize)]
struct TaskParameters {
	repetitions: Option<usize>,
	data_interval: Option<usize>,
	measure_time: Option<bool>,
	save_running_solutions: Option<bool>,
	save_running_fitness: Option<bool>
}

#[derive(Deserialize, Serialize)]
struct AlgoParameters {
	//TODO
}
