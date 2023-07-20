mod app_state;
mod routes;
mod error;
mod models;

use axum;

use app_state::AppState;
use routes::create_routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _app_state = AppState::new();
    let app = create_routes(_app_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
