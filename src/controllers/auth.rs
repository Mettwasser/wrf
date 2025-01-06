#[allow(unused_imports)]
use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::{
        Query,
        State,
    },
    Extension,
};
use cookie::{
    Cookie,
    CookieJar,
};
use loco_rs::{
    controller::bad_request,
    prelude::*,
};
use reqwest::StatusCode;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
};
use time::{
    Duration,
    OffsetDateTime,
};
use uuid::Uuid;

use crate::{
    mailers::auth::AuthMailer,
    models::{
        _entities::prelude::RegisterSessions,
        users::{
            self,
            LoginParams,
            RegisterParams,
        },
    },
    prelude::*,
    views::auth::{
        CurrentResponse,
        LoginResponse,
    },
};
#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyParams {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

/// Result from call to verify the client's response
#[allow(dead_code)]
#[derive(Debug, Default, Deserialize, Clone)]
pub struct CaptchaResponse {
    success: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WarframeProfileResponse {
    pub results: Vec<ProfileResults>,
}

fn deserialize_display_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let mut chars = s.chars();
    chars.next_back();
    Ok(chars.collect())
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ProfileResults {
    #[serde(deserialize_with = "deserialize_display_name")]
    pub display_name: String,
    pub load_out_preset: LoadOutPreset,
}

#[derive(Deserialize)]
struct LoadOutPreset {
    #[serde(rename = "n")]
    pub name: String,
}

#[cfg(not(debug_assertions))]
async fn validate_captcha(params: &RegisterParams, appstate: &AppState) -> Result<()> {
    let form = HashMap::from([
        ("response", params.turnstile_response.as_str()),
        ("secret", &appstate.captcha_secret),
    ]);

    let client = reqwest::Client::new();
    let res = client
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .form(&form)
        .send()
        .await
        .map_err(|err| Error::Any(err.into()))?
        .json::<CaptchaResponse>()
        .await
        .map_err(|err| Error::Any(err.into()))?;

    if !res.success {
        bad_request_v("invalid captcha!")
    } else {
        Ok(())
    }
}

async fn get_warframe_profile(name: &str) -> Result<ProfileResults> {
    let response = reqwest::get(format!(
        "https://content.warframe.com/dynamic/getProfileViewingData.php?n={}",
        name
    ))
    .await
    .map_err(|_| {
        custom::<_, Response>(
            StatusCode::SERVICE_UNAVAILABLE,
            "Failed to contact warframe server",
        )
        .unwrap_err()
    })?;

    match response.status() {
        StatusCode::OK => Ok(response
            .json::<WarframeProfileResponse>()
            .await
            .expect("decode shouldn't fail")
            .results
            .into_iter()
            .next()
            .unwrap()),
        StatusCode::CONFLICT => bad_request_v("username not valid"),
        _ => custom(StatusCode::CONFLICT, "Failed to contact warframe server"),
    }
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    #[allow(unused_variables)] Extension(appstate): Extension<AppState>,
    jar: CookieJar,
    Json(mut params): Json<RegisterParams>,
) -> Result<Response> {
    let Some(cookie) = jar.get("session") else {
        return bad_request_v("no session provided");
    };

    let session_id: Uuid = cookie
        .value()
        .parse()
        .or_else(|_| bad_request_v("couldn't parse cookie's UUID"))?;

    let register_session = RegisterSessions::find_by_session_id(session_id, &ctx.db)
        .await?
        .map_or_else(
            || bad_request_v("Session could not be found. Maybe it's been expired?"),
            Ok,
        )?;

    let profile = get_warframe_profile(&params.name).await?;

    if profile.load_out_preset.name != register_session.verification_code {
        return bad_request_v(
            "Verification Code doesn't match loadout name. Try again in 5-10mins",
        );
    }

    // update the name, because the api is case insensitive (to get the correct name with
    // lower/uppercase)
    params.name = profile.display_name;

    // turnstile validate (disable in debug)
    #[cfg(not(debug_assertions))]
    validate_captcha(&params, &appstate).await?;

    let res = users::Model::create_with_password(&ctx.db, &params).await;

    let user = match res {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &params.email,
                "could not register user",
            );
            return format::json(());
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user).await?;

    format::json(())
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
#[debug_handler]
async fn verify(
    State(ctx): State<AppContext>,
    Query(params): Query<VerifyParams>,
) -> Result<Response> {
    let user = users::Model::find_by_verification_token(&ctx.db, &params.token).await?;

    if user.email_verified_at.is_some() {
        tracing::info!(pid = user.pid.to_string(), "user already verified");
    } else {
        let active_model = user.into_active_model();
        let user = active_model.verified(&ctx.db).await?;
        tracing::info!(pid = user.pid.to_string(), "user verified");
    }

    format::json(())
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn forgot(
    State(ctx): State<AppContext>,
    Json(params): Json<ForgotParams>,
) -> Result<Response> {
    let user = users::Model::find_by_email(&ctx.db, &params.email).await?;

    if user.email_verified_at.is_none() {
        return unauthorized("email not verified!");
    }

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await?;

    AuthMailer::forgot_password(&ctx, &user).await?;

    format::json(())
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Result<Response> {
    let user = users::Model::find_by_reset_token(&ctx.db, &params.token).await?;

    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    format::json(())
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(State(ctx): State<AppContext>, Json(params): Json<LoginParams>) -> Result<Response> {
    let user = users::Model::find_by_email(&ctx.db, &params.email).await?;

    if user.email_verified_at.is_none() {
        return unauthorized("email not verified!");
    }

    if !user.verify_password(&params.password) {
        return bad_request("wrong credentials");
    }

    let jwt_config = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_config.secret, &jwt_config.expiration)
        .or_else(|_| bad_request("unauthorized!"))?;

    let cookie = Cookie::build(("token", &token))
        .http_only(true)
        .expires(OffsetDateTime::now_utc() + Duration::seconds(jwt_config.expiration as i64))
        .secure(cfg!(not(debug_assertions)))
        .path("/")
        .build();

    format::render()
        .cookies(&[cookie])?
        .json(LoginResponse::new(&user, &token))
}

#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/auth")
        .add("/register", post(register))
        // This is a GET to be able to "just click on the link"
        .add("/verify", get(verify))
        .add("/login", post(login))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
        .add("/current", get(current))
}
