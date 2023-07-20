// use sqlx::PgPool;

/*
 * `AppState` represents the application state
 *
 */
#[derive(Clone, Debug)]
pub struct AppState {
    // pool: sqlx::PgPool,
}

impl AppState {
    pub fn new(/* pool: PgPool */) -> Self {
        Self { /* pool */ }
    }
}
