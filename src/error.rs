use axum::http:: StatusCode;
use axum::response::{IntoResponse,Response};
use std::fmt::write;
use serde:: Serialize;


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    // start region: --- Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    // end region: --- Auth errors

    // start region: --- Model errors
    TicketDeleteFailIdNotFound { id: u64 }, 
    // end region: --- Model errors
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        
        response.extensions_mut().insert(self);

        response
    }
}