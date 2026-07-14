mod router;
mod handlers;
mod models;

use models::task::Db;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Shared in-memory store: Arc lets every request handler hold a clone of
    // the pointer; RwLock lets reads run in parallel while writes stay exclusive.
    let db: Db = Arc::new(RwLock::new(std::collections::HashMap::new()));

    let app = router::create_router(db);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind to port 3000");

    println!("listening on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
