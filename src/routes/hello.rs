use crate::{
    app_state::AppState,
    error::Result,
    models::hello::{HelloTable, HelloWorldPayload},
};
use axum::{
    extract::{Path, State},
    response::Json,
};
use serde_json::{json, Value};

#[axum_macros::debug_handler]
pub async fn hello_world(
    Path(user): Path<String>,
    State(state): State<AppState>,
    payload: Json<HelloWorldPayload>,
) -> Result<Json<Value>> {
    tracing::info!("HANDLER -> {:<12}", "/");
    match HelloTable::create(state, user, &payload).await {
        Ok(result) => {
            let response = Json(json!({ "result": result }));
            Ok(response)
        }
        Err(e) => Err(e),
    }
}
