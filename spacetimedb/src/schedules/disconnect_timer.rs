use spacetimedb::ReducerContext;

use crate::utils::lobby_cleanup;

#[spacetimedb::table(accessor = disconnect_timer, private, scheduled(disconnect))]
pub struct DisconnectTimer {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    pub scheduled_at: spacetimedb::ScheduleAt,

    #[unique]
    pub user_id: u32,
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
#[spacetimedb::reducer]
fn disconnect(ctx: &ReducerContext, disconnect_timer: DisconnectTimer) -> Result<(), String> {
    lobby_cleanup(&ctx.db, disconnect_timer.user_id);

    Ok(())
}
