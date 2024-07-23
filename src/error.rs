use std::fmt;
use std::fmt::{Display, Formatter};
use actix_web::{Error, HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use serde::{ Serialize};



#[derive(Serialize,Debug)]
pub enum ServerError{
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug,Serialize)]
pub struct ServerErrorResponse{
    error_message:String,
}

impl ServerError {
    fn error_response(&self) -> String {
        match self {
            ServerError::DBError(msg) => {
                log::error!("DB Error: {}", msg);
                "Database Error".into()},
            ServerError::ActixError(msg) => {
                log::error!("Actix Error: {}", msg);
                "Internal Server error".into()
            },
            ServerError::NotFound(msg) =>  msg.into(),
        }
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}",self)
    }
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::DBError(_) | ServerError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ServerErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl From<actix_web::error::Error> for ServerError{
    fn from(value: Error) -> Self {
        println!("调用了actix_web::error::Error 的from方法");
        ServerError::ActixError(value.to_string())
    }




}
impl From<sqlx::error::Error> for ServerError {
    fn from(value: sqlx::Error) -> Self {
        println!("调用了sqlx::error::Error 的from方法");
        ServerError::DBError(value.to_string())
    }
}