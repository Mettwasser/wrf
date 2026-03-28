use spacetimedb::{
    Identity,
    Local,
    ProcedureContext,
    ReducerContext,
    Table,
    TryInsertError,
    ViewContext,
};

use crate::{
    error::USER_NOT_CREATED,
    model::{
        allowlist,
        lobby,
        lobby_ban,
        lobby_join,
        user_details,
        user_id,
        user_id__view,
    },
    permissions::Permissions,
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

pub fn remove_player_from_lobby(db: &Local, user_id: u32) {
    if let Some(lobby_id) = db
        .lobby_join()
        .user_id()
        .find(user_id)
        .map(|lobby| lobby.lobby_id)
        && let Some(mut lobby) = db.lobby().lobby_id().find(lobby_id)
    {
        lobby.amount_players -= 1;
        db.lobby().lobby_id().update(lobby);
        db.lobby_join().user_id().delete(user_id);
    }
}

pub fn lobby_cleanup(db: &Local, user_id: u32) {
    // Lobby HOST cleanup
    db.lobby().lobby_id().delete(user_id);
    db.lobby_ban().lobby_id().delete(user_id);
    db.lobby_join().lobby_id().delete(user_id);

    // Lobby join cleanup
    remove_player_from_lobby(db, user_id);
}

fn has_perms(
    identity: Identity,
    permissions_to_check: Permissions,
    db: &Local,
) -> Result<bool, String> {
    let Some(user_id) = db.user_id().identity().find(identity).map(|row| row.id) else {
        if db.allowlist().id().find(identity).is_some() {
            return Ok(true);
        }
        return Err(USER_NOT_CREATED.to_owned());
    };

    let has_permission = db
        .user_details()
        .user_id()
        .find(user_id)
        .is_some_and(|details| details.permissions.contains(permissions_to_check));

    Ok(has_permission || db.allowlist().id().find(identity).is_some())
}

pub trait UserCtx: Sized {
    fn user_id(self) -> Option<u32>;

    fn user_id_or_err(self) -> Result<u32, String> {
        self.user_id().ok_or_else(|| USER_NOT_CREATED.to_owned())
    }
}

pub trait PermissionsCtx: UserCtx {
    fn require_permissions(self, permissions: Permissions) -> Result<(), String>;
}

impl UserCtx for &ReducerContext {
    fn user_id(self) -> Option<u32> {
        self.db
            .user_id()
            .identity()
            .find(self.sender())
            .map(|row| row.id)
    }
}

impl PermissionsCtx for &ReducerContext {
    fn require_permissions(self, permissions_to_check: Permissions) -> Result<(), String> {
        has_perms(self.sender(), permissions_to_check, &self.db)?
            .then_some(())
            .ok_or_else(|| "Insufficient permissions".to_owned())
    }
}

impl UserCtx for &mut ProcedureContext {
    fn user_id(self) -> Option<u32> {
        let sender = self.sender();

        self.with_tx(|ctx| ctx.db.user_id().identity().find(sender).map(|row| row.id))
    }
}

impl PermissionsCtx for &mut ProcedureContext {
    fn require_permissions(self, permissions_to_check: Permissions) -> Result<(), String> {
        let sender = self.sender();

        self.with_tx(|ctx| {
            has_perms(sender, permissions_to_check, &ctx.db)?
                .then_some(())
                .ok_or_else(|| "Insufficient permissions".to_owned())
        })
    }
}

impl UserCtx for &ViewContext {
    fn user_id(self) -> Option<u32> {
        self.db
            .user_id()
            .identity()
            .find(self.sender())
            .map(|row| row.id)
    }
}
