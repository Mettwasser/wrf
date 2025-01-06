use chrono::{
    Duration,
    Local,
};
use cookie::{
    Cookie,
    CookieJar,
};
use loco_rs::{
    controller::bad_request,
    prelude::*,
};

use crate::{
    models::_entities::register_sessions::{
        ActiveModel,
        Entity,
    },
    naive_to_offset_datetime,
    views::register_sessions::CurrentResponse,
};

pub async fn create_new(ctx: AppContext) -> Result<Response> {
    let uuid = Uuid::new_v4();
    let code: String = Uuid::new_v4().to_string().chars().take(8).collect();
    let expiry = Local::now().naive_local() + Duration::days(1);

    let item = ActiveModel {
        expiry: Set(expiry),
        verification_code: Set(code),
        session_id: Set(uuid),

        ..Default::default()
    };

    let item = item.insert(&ctx.db).await?;

    let session_cookie = Cookie::build(("session", uuid.to_string()))
        .http_only(true)
        .expires(naive_to_offset_datetime(expiry))
        .secure(cfg!(not(debug_assertions)))
        .path("/")
        .build();

    format::render()
        .cookies(&[session_cookie])?
        .json(CurrentResponse::from(item))
}

fn parse_session(cookie_value: &str) -> Result<Uuid> {
    cookie_value
        .parse()
        .or_else(|_| bad_request("couldn't parse cookie's UUID"))
}

pub async fn current(jar: CookieJar, State(ctx): State<AppContext>) -> Result<Response> {
    match jar.get("session") {
        Some(cookie) => match parse_session(cookie.value()) {
            Ok(session_id) => {
                if let Some(item) = Entity::find_by_session_id(session_id, &ctx.db).await? {
                    format::json(CurrentResponse::from(item))
                } else {
                    create_new(ctx).await
                }
            }
            Err(err) => Err(err),
        },
        None => create_new(ctx).await,
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/register_sessions/")
        .add("/current", get(current))
}
