use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use eas::algorithms::EvolutionaryAlgorithm;
use rand::rng;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::create::create_ea;
use crate::schedule::TaskSchedule;
use crate::{SharedState, Task};

pub async fn handle_websocket_connect(
    State(state): State<SharedState>,
    ws: WebSocketUpgrade,
    Path(id): Path<Uuid>,
) -> Response {
    let Some(schedule) = state
        .lock()
        .expect("failed to aquire lock")
        .pending_schedules
        .remove(&id)
    else {
        return StatusCode::NOT_FOUND.into_response();
    };
    return ws.on_upgrade(move |socket| handle_schedule(socket, schedule));
}

async fn handle_schedule(mut socket: WebSocket, schedule: TaskSchedule) {
    println!("here!");
    for task in schedule.tasks {
        for _ in 0..schedule.repeat_count {
            let Ok(mut runner) = create_ea(&task) else {
                return;
            };
            let _ = send_json(
                &mut socket,
                json!({
                    "messageType": "setTask",
                    "task": task
                }
                ),
            )
            .await;
            run_task(&task, &mut runner, &mut socket).await;
            let _ = send_json(
                &mut socket,
                json!({
                    "messageType": "result",
                    "result": {
                        "task": task,
                        "iterations": runner.iterations(),
                        "fitness": runner.current_fitness(),
                    },
                }),
            )
            .await;
        }
        println!("done!");
    }
}

async fn send_json(socket: &mut WebSocket, value: Value) -> Result<(), axum::Error> {
    let message_text = serde_json::to_string(&value).unwrap();
    socket.send(Message::Text(message_text.into())).await
}

async fn run_task(
    task: &Task,
    runner: &mut Box<dyn EvolutionaryAlgorithm + Send>,
    socket: &mut WebSocket,
) {
    let _ = send_json(
        socket,
        json!({
            "messageType": "dataUpdate",
            "data": runner.status_json(),
        }),
    )
    .await;
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
            let _ = send_json(
                socket,
                json!({
                    "messageType": "dataUpdate",
                    "data": runner.status_json(),
                }),
            )
            .await;
        }
    }
    let _ = send_json(
        socket,
        json!({
            "messageType": "dataUpdate",
            "data": runner.status_json(),
        }),
    )
    .await;
}
