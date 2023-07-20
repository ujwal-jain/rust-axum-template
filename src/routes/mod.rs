pub mod hello; 

use crate::app_state;
use axum::middleware::map_response;
use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use crate::routes::hello::hello_world;

pub fn create_routes(state: app_state::AppState) -> Router<(), Body> {
    let routes = Router::new()
        .route("/:user", get(hello_world));

    Router::new()
        .merge(routes)
        .layer(map_response(main_response_mapper))
        .with_state(state)
        .fallback(handler_404)
}

async fn main_response_mapper(res: Response) -> Response {
    tracing::debug!("{:<12} [main_response_mapper]\n", "RES_MAPPER");
    res
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "No route found")
}
