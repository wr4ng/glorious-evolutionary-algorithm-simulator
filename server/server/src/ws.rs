use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use eas::algorithms::Algorithm;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use rand::SeedableRng;
use rand_pcg::Pcg64;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::create::create_ea;
use crate::schedule::TaskSchedule;
use crate::{SharedState, Task};

// Handle initial connection of websocket
// Checks if the given ID matches a pending TaskSchedule
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
    ws.on_upgrade(move |socket| handle_schedule(socket, schedule))
}

// Run a TaskSchedule, periodically sending data updates on the WebSocket connection
async fn handle_schedule(socket: WebSocket, schedule: TaskSchedule) {
    println!("[{}] schedule execution started", schedule.id);

    // Split socket into a transmit and receive part
    let (mut tx, mut rx) = socket.split();

    // Receive part is used to detect if client disconnects early,
    // and stop the simulation if they do
    let mut receive_task =
        tokio::spawn(async move { while let Some(Ok(Message::Text(_))) = rx.next().await {} });

    // Task running the actual simulation
    let mut simulation_task = tokio::spawn(async move {
        // Use a seeded RNG to be able to reproduce results given the same seed and schedule
        let mut rng = Pcg64::seed_from_u64(schedule.seed);

        // Loop over each task and perform them the given amount of times
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
                // Run task until a stopping criteria is met
                run_task(&task, schedule.update_rate, &mut runner, &mut rng, &mut tx).await;
                // Send resulting task data
                if send_json(
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
                .is_err()
                {
                    return;
                }
            }
        }
        println!("[{}] schedule completed successfully", schedule.id);
    });

    // Run both receive and simulation task until either one finishes
    tokio::select! {
        _ = &mut receive_task => {
                println!("[{}] schedule aborted early", schedule.id);
                simulation_task.abort()
        },
        _ = &mut simulation_task => receive_task.abort(),
    }

    println!("[{}] schedule done", schedule.id);
}

// Send a JSON value as text over WebSocket connection
async fn send_json(
    socket: &mut SplitSink<WebSocket, Message>,
    value: Value,
) -> Result<(), axum::Error> {
    let message_text = serde_json::to_string(&value).unwrap();
    socket.send(Message::Text(message_text.into())).await
}

// Run a task until a stopping criteria is met
// Sends simulation status periodically based on update_rate
async fn run_task(
    task: &Task,
    update_rate: u64,
    runner: &mut Box<dyn Algorithm<Pcg64>>,
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
            if optimal == runner.current_fitness() {
                break;
            }
        }
        // Every update_rate iterations, send a data update to the client
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
            let _ = socket.flush().await;
        }
    }
    // Send a final data update once the simulation is done
    let _ = send_json(
        socket,
        json!({
            "messageType": "dataUpdate",
            "data": runner.status_json(),
        }),
    )
    .await;
}
