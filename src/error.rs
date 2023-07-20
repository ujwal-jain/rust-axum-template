use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

/*
 * `Error` represents the custom error type for this module, encompassing various error scenarios.
 *
 * Variants:
 */
#[derive(Debug)]
pub enum Error {
    // ExampleError { e: ExampleErrorType } 
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("Error: {self:?}");
        match self {
            //Error::ExampleError { error } => (
                //StatusCode::INTERNAL_SERVER_ERROR,
                //format!("ExampleError: {:?}", error),
            //)
                //.into_response(),
        }
    }
}


