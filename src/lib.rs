use std::sync::Arc;

use chrono::{
    NaiveDateTime,
    TimeZone,
    Timelike,
    Utc,
};
use loco_rs::{
    controller::ErrorDetail,
    Error,
    Result,
};
use reqwest::StatusCode;
use time::OffsetDateTime;

pub mod app;
pub mod controllers;
pub mod initializers;
pub mod mailers;
pub mod models;
pub mod tasks;
pub mod views;
pub mod workers;

pub type Relics = Arc<Vec<String>>;

fn naive_to_offset_datetime(naive_datetime: NaiveDateTime) -> OffsetDateTime {
    // Convert NaiveDateTime to DateTime<Utc>
    let datetime_utc: chrono::DateTime<Utc> = Utc.from_utc_datetime(&naive_datetime);

    // Extract seconds and nanoseconds from DateTime<Utc>
    let timestamp_secs = datetime_utc.timestamp();
    let timestamp_nanos = datetime_utc.nanosecond();

    // Create OffsetDateTime from Unix timestamp and set the nanoseconds
    OffsetDateTime::from_unix_timestamp(timestamp_secs)
        .expect("Failed to convert Unix timestamp")
        .replace_nanosecond(timestamp_nanos)
        .expect("Failed to set nanoseconds")
}

// _v means visible; the errors are visible to the client
pub fn bad_request_v<T: Into<String>, U>(msg: T) -> Result<U> {
    Err(Error::CustomError(
        StatusCode::BAD_REQUEST,
        ErrorDetail {
            description: Some(msg.into()),
            error: Some("bad_request".to_owned()),
        },
    ))
}

pub fn unauthorized_v<T: Into<String>, U>(msg: T) -> Result<U> {
    Err(Error::CustomError(
        StatusCode::UNAUTHORIZED,
        ErrorDetail {
            description: Some(msg.into()),
            error: Some("unauthorized".to_owned()),
        },
    ))
}

pub fn conflict_v<T: Into<String>, U>(msg: T) -> Result<U> {
    Err(Error::CustomError(
        StatusCode::CONFLICT,
        ErrorDetail {
            description: Some(msg.into()),
            error: Some("conflict".to_owned()),
        },
    ))
}

pub fn internal_server_error_v<T: Into<String>, U>(msg: T) -> Result<U> {
    Err(Error::CustomError(
        StatusCode::INTERNAL_SERVER_ERROR,
        ErrorDetail {
            description: Some(msg.into()),
            error: Some("internal_server_error".to_owned()),
        },
    ))
}

pub fn custom<T: Into<String>, U>(code: StatusCode, msg: T) -> Result<U> {
    Err(Error::CustomError(
        code,
        ErrorDetail {
            description: Some(msg.into()),
            error: Some(
                code.canonical_reason()
                    .expect("Code should be valid")
                    .to_lowercase()
                    .replace(" ", "_"),
            ),
        },
    ))
}

pub mod prelude {
    pub use crate::{
        app::AppState,
        bad_request_v,
        conflict_v,
        custom,
        internal_server_error_v,
        unauthorized_v,
    };
}
