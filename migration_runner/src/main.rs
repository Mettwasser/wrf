use crate::module_bindings::{DbConnection, OldUser, migrate_user_elevated};
use serde::{Deserialize, Deserializer, Serialize};
use spacetimedb_sdk::Identity;
use std::{
    env,
    fs::{self, DirEntry},
};

mod module_bindings;

type BoxDynError = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "deserialize_identity")]
    id: Identity,

    username: String,

    verified: bool,

    is_admin: bool,
}

fn deserialize_identity<'de, D>(deserializer: D) -> Result<Identity, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct IdentityWrapper {
        __identity__: String,
    }

    let wrapper = IdentityWrapper::deserialize(deserializer)?;
    let hex = wrapper
        .__identity__
        .strip_prefix("0x")
        .unwrap_or(&wrapper.__identity__);

    Identity::from_hex(hex).map_err(serde::de::Error::custom)
}

fn get_file() -> Result<DirEntry, BoxDynError> {
    let file = fs::read_dir("exports\n")?
        .next()
        .ok_or("No file in exports found")??;

    if !file.file_type()?.is_file() {
        return Err("Not a file".into());
    }

    Ok(file)
}

fn create_conn() -> Result<DbConnection, BoxDynError> {
    dotenvy::from_filename(".env")?;

    let uri = if cfg!(feature = "remote") {
        env::var("STDB_URL_REMOTE")?
    } else {
        env::var("STDB_URL_LOCAL")?
    };

    let db_name = env::var("STDB_DB_NAME")?;
    let token = env::var("STDB_TOKEN")?;

    let conn = DbConnection::builder()
        .with_uri(uri)
        .with_database_name(db_name)
        .with_token(Some(token))
        .build()?;

    conn.run_threaded();

    Ok(conn)
}

fn main() -> Result<(), BoxDynError> {
    let conn = create_conn()?;
    let file = get_file()?;
    let json = fs::read_to_string(file.path())?;

    let users: Vec<User> = serde_json::from_str(&json)?;

    for user in users {
        let old_user = OldUser {
            id: user.id,
            username: user.username,
            verified: user.verified,
        };

        println!("Inserting {old_user:?}");

        conn.reducers.migrate_user_elevated(old_user)?;
    }

    Ok(())
}
