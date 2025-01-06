// auth mailer
#![allow(non_upper_case_globals)]

use loco_rs::prelude::*;
use serde_json::json;

use crate::models::users;

static welcome: Dir<'_> = include_dir!("src/mailers/auth/welcome");
static forgot: Dir<'_> = include_dir!("src/mailers/auth/forgot");
// #[derive(Mailer)] // -- disabled for faster build speed. it works. but lets
// move on for now.

#[allow(clippy::module_name_repetitions)]
pub struct AuthMailer {}
impl Mailer for AuthMailer {
    fn opts() -> mailer::MailerOpts {
        mailer::MailerOpts {
            from: "wrf-noreply@mettwasser.xyz".to_owned(),
            ..Default::default()
        }
    }
}
impl AuthMailer {
    /// Sending welcome email the the given user
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_welcome(ctx: &AppContext, user: &users::Model) -> Result<()> {
        Self::mail_template(
            ctx,
            &welcome,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "verifyToken": user.email_verification_token,
                  "domain": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    /// Sending forgot password email
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn forgot_password(ctx: &AppContext, user: &users::Model) -> Result<()> {
        let domain = ctx
            .config
            .settings
            .as_ref()
            .and_then(|settings| {
                settings
                    .get("frontend_url")?
                    .as_str()
                    // for whatever reason it's quoted
                    .map(|s| s.replace("\"", ""))
            })
            .unwrap_or_else(|| ctx.config.server.full_url());

        Self::mail_template(
            ctx,
            &forgot,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "resetToken": user.reset_token,
                  "domain": domain
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }
}
