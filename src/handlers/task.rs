use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::models::task::{CreateTask, Db, Task, UpdateTask};

/// POST /tasks
/// Create a new task and return it with a 201 status.
pub async fn create_task(
    State(db): State<Db>,
    Json(payload): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    let task = Task {
        id: Uuid::new_v4(),
        title: payload.title,
        done: false,
    };

    // Scope the lock tightly: acquire, insert, clone what we need, drop.
    db.lock().unwrap().insert(task.id, task.clone());

    (StatusCode::CREATED, Json(task))
}

/// GET /tasks
/// Return every task currently stored.
pub async fn list_tasks(State(db): State<Db>) -> Json<Vec<Task>> {
    let tasks = db.lock().unwrap().values().cloned().collect::<Vec<_>>();
    Json(tasks)
}

/// GET /tasks/:id
/// Return a single task, or 404 if the id isn't found.
pub async fn get_task(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, StatusCode> {
    db.lock()
        .unwrap()
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// PUT /tasks/:id
/// Update an existing task's title and/or done flag. 404 if not found.
pub async fn update_task(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, StatusCode> {
    let mut tasks = db.lock().unwrap();

    let task = tasks.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = payload.title {
        task.title = title;
    }
    if let Some(done) = payload.done {
        task.done = done;
    }

    Ok(Json(task.clone()))
}

/// DELETE /tasks/:id
/// Remove a task. 204 on success, 404 if it never existed.
pub async fn delete_task(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let removed = db.lock().unwrap().remove(&id);

    match removed {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND,
    }
}
