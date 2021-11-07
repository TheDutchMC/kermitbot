use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;
use log::warn;

pub type HttpResult = Result<HttpResponse, Error>;

#[derive(Serialize)]
struct ErrorResponse {
    code:       u16,
    message:    String
}

impl From<&Error> for ErrorResponse {
    fn from(e: &Error) -> Self {
        Self {
            code: e.status_code().as_u16(),
            message: format!("{}", e)
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal Server Error")]
    Mysql(#[from] mysql::Error),
    #[error("Internal Server Error")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    #[allow(unreachable_patterns)]
    fn log(&self) {
        match self {
            Self::Mysql(e) => warn!("{:?}", e),
            Self::Anyhow(e) => warn!("{:?}", e),
            _ => {}
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Mysql(_)  | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        self.log();
        let er = ErrorResponse::from(self);
        HttpResponse::build(self.status_code()).json(&er)
    }
}