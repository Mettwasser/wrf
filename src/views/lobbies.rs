use serde::Serialize;

use crate::models::{
    _entities::lobbies,
    users,
};

#[derive(Debug, Serialize)]
pub struct LobbyCreateResponse {
    pub user: users::Model,
    pub lobby: lobbies::Model,
}
