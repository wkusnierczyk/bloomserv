// src/main.rs
use std::net::SocketAddr;
// Use the logic from lib.rs
// If your Cargo.toml name is "bloom_daemon", use that here.
use bloomsrv::{create_app, SharedState};

#[tokio::main]
async fn main() {
    let state = SharedState::default();

    // We use the public function from lib.rs
    let app = create_app(state);

    let host = [127, 0, 0, 1];
    let port = 3000;
    let addr = SocketAddr::from((host, port));
    println!("Bloom Daemon listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
