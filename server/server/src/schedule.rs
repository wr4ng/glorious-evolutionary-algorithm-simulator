use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{SharedState, Task, create::create_ea};

#[derive(Deserialize)]
pub struct CreateTaskScheduleRequest {
    tasks: Vec<Task>,
    repeat_count: usize,
}

#[derive(Serialize, Clone)]
pub struct TaskSchedule {
    pub id: Uuid,
    pub tasks: Vec<Task>,
    pub repeat_count: usize,
}

//TODO: Error type
pub async fn create_task_schedule(
    State(state): State<SharedState>,
    Json(request): Json<CreateTaskScheduleRequest>,
) -> Result<Json<TaskSchedule>, StatusCode> {
    let schedule_id = Uuid::new_v4();

    //TODO: Better error
    if request.tasks.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    //TODO: Better error
    if request.repeat_count == 0 || request.repeat_count > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }

    for task in &request.tasks {
        if create_ea(task).is_err() {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let schedule = TaskSchedule {
        id: schedule_id,
        tasks: request.tasks,
        repeat_count: request.repeat_count,
    };
    let schedule_result = schedule.clone();

    state
        .lock()
        .expect("failed to aquire lock")
        .pending_schedules
        .insert(schedule_id, schedule);

    Ok(Json(schedule_result))
}
