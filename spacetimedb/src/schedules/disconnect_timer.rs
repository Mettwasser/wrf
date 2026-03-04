use spacetimedb::{
    Identity,
    ReducerContext,
};

use crate::utils::lobby_cleanup;

#[spacetimedb::table(accessor = disconnect_timer, private, scheduled(disconnect))]
pub struct DisconnectTimer {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    pub scheduled_at: spacetimedb::ScheduleAt,

    #[unique]
    pub user: Identity,
}

#[spacetimedb::reducer]
fn disconnect(ctx: &ReducerContext, disconnect_timer: DisconnectTimer) -> Result<(), String> {
    lobby_cleanup(&ctx.db, disconnect_timer.user);

    Ok(())
}
