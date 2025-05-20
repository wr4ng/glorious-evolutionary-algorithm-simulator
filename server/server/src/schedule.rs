use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{SharedState, Task, create::create_ea};

#[derive(Deserialize)]
pub struct CreateTaskScheduleRequest {
    tasks: Vec<Task>,
    repeat_count: u64,
    update_rate: u64,
    seed: u64,
}

#[derive(Serialize, Clone)]
pub struct TaskSchedule {
    pub id: Uuid,
    pub tasks: Vec<Task>,
    pub repeat_count: u64,
    pub update_rate: u64,
    pub seed: u64,
}

//TODO: Error type
pub async fn create_task_schedule(
    State(state): State<SharedState>,
    Json(request): Json<CreateTaskScheduleRequest>,
) -> Result<Json<TaskSchedule>, StatusCode> {
    let schedule_id = Uuid::new_v4();

    // Validate repeat_count
    //TODO: Better error
    //TODO: Determine ranges
    if request.repeat_count == 0 || request.repeat_count > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate update_rate
    //TODO: Better error
    //TODO: Determine ranges
    if request.update_rate < 1000 || request.update_rate > 100000 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate tasks
    //TODO: Better error
    if request.tasks.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    for task in &request.tasks {
        if create_ea(task, &mut rand::rng()).is_err() {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let schedule = TaskSchedule {
        id: schedule_id,
        tasks: request.tasks,
        repeat_count: request.repeat_count,
        update_rate: request.update_rate,
        seed: request.seed,
    };
    let schedule_result = schedule.clone();

    state
        .lock()
        .expect("failed to aquire lock")
        .pending_schedules
        .insert(schedule_id, schedule);

    println!("[{}] schedule created", schedule_id);
    Ok(Json(schedule_result))
}
