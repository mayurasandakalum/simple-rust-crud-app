use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// The resource this CRUD API manages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub done: bool,
}

/// Fields accepted when creating a task. No `id` or `done` — the server
/// assigns the id and defaults `done` to false.
#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub title: String,
}

/// Fields accepted when updating a task. Both optional so a PUT can change
/// just the title, just the done flag, or both.
#[derive(Debug, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub done: Option<bool>,
}

/// Shared application state: a thread-safe, reference-counted map of
/// task id -> Task. Arc allows cheap clones across handlers; RwLock
/// lets any number of readers proceed in parallel while writers get
/// exclusive access.
pub type Db = Arc<RwLock<HashMap<Uuid, Task>>>;
