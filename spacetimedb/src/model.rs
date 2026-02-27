use spacetimedb::Identity;

#[spacetimedb::table(accessor = user, public)]
pub struct User {
    #[primary_key]
    pub id: Identity,

    #[unique]
    pub username: String,

    pub verified: bool,
}

#[spacetimedb::table(accessor = user_verification)]
pub struct UserVerification {
    #[primary_key]
    pub id: Identity,
    pub code: String,
    pub warframe_id: Option<String>,
}
