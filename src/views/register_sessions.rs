use derive_more::derive::From;
use loco_rs::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::models::_entities::register_sessions::Model;

#[derive(Debug, Serialize, From)]
#[serde(rename_all = "camelCase")]
pub struct CurrentResponse {
    pub session_id: Uuid,
    pub expiry: DateTime,
    pub verification_code: String,
}

impl From<Model> for CurrentResponse {
    fn from(
        Model {
            session_id,
            expiry,
            verification_code,
            ..
        }: Model,
    ) -> Self {
        Self {
            expiry,
            session_id,
            verification_code,
        }
    }
}
