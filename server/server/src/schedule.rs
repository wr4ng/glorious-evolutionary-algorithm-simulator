use std::{
    thread::{self, sleep},
    time::Duration,
};

use axum::{Json, extract::State, http::StatusCode};
use eas::algorithms::EvolutionaryAlgorithm;
use rand::rng;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::broadcast::{Sender, channel};
use uuid::Uuid;

use crate::{ScheduleResult, SharedState, Task, TaskResult, create::create_ea};

#[derive(Deserialize)]
pub struct CreateTaskScheduleRequest {
    tasks: Vec<Task>,
    repeat_count: usize,
}

#[derive(Serialize, Clone)]
pub struct TaskSchedule {
    id: Uuid,
    tasks: Vec<Task>,
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

    let schedule = TaskSchedule {
        id: schedule_id,
        tasks: request.tasks,
    };
    let schedule_result = schedule.clone();

    // Parse and map each task to a runner,
    // returning the first error if any is found
    let runners = match schedule
        .tasks
        .into_iter()
        .map(|t| {
            (
                t.clone(),
                (0..request.repeat_count)
                    .map(|_| create_ea(&t))
                    .collect::<Result<Vec<_>, _>>(),
            )
        })
        .map(|(t, r)| r.map(|r| (t, r)))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    thread::spawn(move || {
        let (tx, _) = channel::<serde_json::Value>(100); //TODO: Determine capacity
        state
            .lock()
            .expect("failed to aquire mutex")
            .schedule_channels
            .insert(schedule.id, tx.clone());

        // Give time for client to connect to WebSocket before starting
        sleep(Duration::from_millis(100));

        let mut results = Vec::new();
        for (task, task_runners) in runners {
            for mut runner in task_runners {
                let _ = tx.send(json!({
                    "messageType": "setTask",
                    "task": task
                }
                ));

                run_task(&task, &mut runner, &tx);

                let _ = tx.send(json!({
                    "messageType": "result",
                    "result": {
                        "task": task,
                        "iterations": runner.iterations(),
                        "fitness": runner.current_fitness(),
                    },
                }));

                let task_result = TaskResult {
                    task: task.clone(),
                    iterations: runner.iterations(),
                    fitness: runner.current_fitness(),
                };
                results.push(task_result);
            }
        }

        // Remove channel + add results
        {
            let mut state = state.lock().expect("failed to aquire mutex");
            state.schedule_channels.remove(&schedule.id);
            state.results.push(ScheduleResult { results });
        }
    });

    Ok(Json(schedule_result))
}

fn run_task(task: &Task, runner: &mut Box<dyn EvolutionaryAlgorithm + Send>, tx: &Sender<Value>) {
    let _ = tx.send(json!({
        "messageType": "dataUpdate",
        "data": runner.status_json(),
    }));
    loop {
        runner.iterate(&mut rng());
        if runner.iterations() >= task.stop_cond.max_iterations {
            break;
        }
        // If optimal solution is provided, stop if it is reached
        if let Some(optimal) = task.stop_cond.optimal_fitness {
            //TODO: Use fitness function to compare to avoid exact matching
            if optimal == runner.current_fitness() {
                break;
            }
        }
        //TODO: Don't use fixed update-rate
        if runner.iterations() % 1000 == 0 {
            let _ = tx.send(json!({
                "messageType": "dataUpdate",
                "data": runner.status_json(),
            }));
        }
    }
    let _ = tx.send(json!({
        "messageType": "dataUpdate",
        "data": runner.status_json(),
    }));
}
