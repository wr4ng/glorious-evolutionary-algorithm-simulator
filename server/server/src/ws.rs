use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use eas::algorithms::EvolutionaryAlgorithm;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use rand::SeedableRng;
use rand_pcg::Pcg64;
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

async fn handle_schedule(socket: WebSocket, schedule: TaskSchedule) {
    println!("[{}] schedule execution started", schedule.id);

    let (mut tx, mut rx) = socket.split();

    let mut receive_task =
        tokio::spawn(async move { while let Some(Ok(Message::Text(_))) = rx.next().await {} });

    let mut simulation_task = tokio::spawn(async move {
        let mut rng = Pcg64::seed_from_u64(schedule.seed);

        for task in schedule.tasks {
            for _ in 0..schedule.repeat_count {
                let Ok(mut runner) = create_ea(&task, &mut rng) else {
                    return;
                };

                // Send initial task data
                let _ = send_json(
                    &mut tx,
                    json!({
                        "messageType": "setTask",
                        "task": task
                    }
                    ),
                )
                .await;

                run_task(&task, schedule.update_rate, &mut runner, &mut rng, &mut tx).await;

                if let Err(_) = send_json(
                    &mut tx,
                    json!({
                        "messageType": "result",
                        "result": {
                            "task": task,
                            "iterations": runner.iterations(),
                            "fitness": runner.current_fitness(),
                        },
                    }),
                )
                .await
                {
                    println!("[{}] schedule aborted early", schedule.id);
                    return;
                }
            }
        }
        println!("[{}] schedule completed", schedule.id);
    });

    tokio::select! {
        _ = &mut receive_task => simulation_task.abort(),
        _ = &mut simulation_task => receive_task.abort(),
    }
}

async fn send_json(
    socket: &mut SplitSink<WebSocket, Message>,
    value: Value,
) -> Result<(), axum::Error> {
    let message_text = serde_json::to_string(&value).unwrap();
    socket.send(Message::Text(message_text.into())).await
}

async fn run_task(
    task: &Task,
    update_rate: u64,
    runner: &mut Box<dyn EvolutionaryAlgorithm<Pcg64>>,
    rng: &mut Pcg64,
    socket: &mut SplitSink<WebSocket, Message>,
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
        runner.iterate(rng);
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
        if runner.iterations() % update_rate == 0 {
            if let Err(_) = send_json(
                socket,
                json!({
                    "messageType": "dataUpdate",
                    "data": runner.status_json(),
                }),
            )
            .await
            {
                return;
            }
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
