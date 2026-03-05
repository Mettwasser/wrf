use std::fmt::Display;

use spacetimedb::{
    Identity,
    Local,
    Table,
    TryInsertError,
};

use crate::model::{
    lobby,
    lobby_ban,
    lobby_join,
};

pub trait MapUniqueViolation<T, F> {
    fn map_unique_violation(self, map: F) -> Result<T, String>;
}

impl<T, F, R> MapUniqueViolation<T::Row, F> for Result<T::Row, TryInsertError<T>>
where
    T: Table,
    F: FnOnce(T::UniqueConstraintViolation) -> R,
    R: Into<String>,
{
    fn map_unique_violation(self, map: F) -> Result<T::Row, String> {
        self.map_err(|e| match e {
            TryInsertError::UniqueConstraintViolation(err) => map(err).into(),
            TryInsertError::AutoIncOverflow(err) => err.to_string(),
        })
    }
}

pub trait ErrorToString<T> {
    fn error_as_string(self) -> Result<T, String>;
}

impl<T, E> ErrorToString<T> for Result<T, E>
where
    E: Display,
{
    fn error_as_string(self) -> Result<T, String> {
        self.map_err(|err| err.to_string())
    }
}

pub fn remove_player_from_lobby(db: &Local, user_id: Identity) {
    if let Some(lobby_id) = db.lobby_join().user().find(user_id).map(|lobby| lobby.host)
        && let Some(mut lobby) = db.lobby().host().find(lobby_id)
    {
        lobby.amount_players -= 1;
        db.lobby().host().update(lobby);
        db.lobby_join().user().delete(user_id);
    }
}

pub fn lobby_cleanup(db: &Local, user_id: Identity) {
    // Lobby HOST cleanup
    db.lobby().host().delete(user_id);
    db.lobby_ban().host().delete(user_id);
    db.lobby_join().host().delete(user_id);

    // Lobby join cleanup
    remove_player_from_lobby(db, user_id);
}
