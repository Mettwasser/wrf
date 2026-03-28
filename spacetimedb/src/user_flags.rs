use bitmask_enum::bitmask;
use spacetimedb::SpacetimeType;

#[bitmask(u64)]
#[derive(SpacetimeType)]
pub enum UserFlags {
    Verified,
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
