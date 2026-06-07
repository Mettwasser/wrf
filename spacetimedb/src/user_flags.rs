use bitmask_enum::bitmask;
use spacetimedb::SpacetimeType;

#[bitmask(u64)]
#[derive(SpacetimeType)]
pub enum UserFlags {
    Verified,
}

impl Default for UserFlags {
    fn default() -> Self {
        Self::none()
    }
}

impl UserFlags {
    #[must_use]
    pub const fn from_verified(verified: bool) -> Self {
        if verified {
            Self::Verified
        } else {
            Self::none()
        }
    }
}
