use serde::Deserialize;

use crate::error::Result;

use crate::app_state::AppState;

use std::{thread, time};

#[derive(Debug, Deserialize)]
pub struct HelloWorldPayload {}

#[derive(Debug)]
pub struct HelloTable {}

impl HelloTable {
    pub async fn create(_: AppState, user: String, _: &HelloWorldPayload) -> Result<String> {
        let ten_millis = time::Duration::from_millis(10);
        let now = time::Instant::now();
        thread::sleep(ten_millis);
        assert!(now.elapsed() >= ten_millis);
        Ok(format!("Hello, {}!", user))
    }
}

