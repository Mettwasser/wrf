use bitmask_enum::bitmask;
use spacetimedb::SpacetimeType;

#[bitmask(u128)]
#[derive(SpacetimeType)]
pub enum Permissions {
    CREATE_LOBBY,
    JOIN_LOBBY,

    SET_USERNAME,

    MANAGE_USER_FLAGS,

    MANAGE_MODERATOR,

    MANAGE_ADMIN,
}

mod roles {
    use crate::permissions::Permissions;

    pub const USER_ROLE: Permissions = Permissions::CREATE_LOBBY
        .or(Permissions::JOIN_LOBBY)
        .or(Permissions::SET_USERNAME);

    pub const MOD_ROLE: Permissions = USER_ROLE.or(Permissions::MANAGE_USER_FLAGS);

    pub const ADMIN_ROLE: Permissions = MOD_ROLE.or(Permissions::MANAGE_MODERATOR);

    pub const OWNER_ROLE: Permissions = ADMIN_ROLE.or(Permissions::MANAGE_ADMIN);
}

impl Permissions {
    #[must_use]
    pub fn for_role(role: Role) -> Self {
        role.into_permissions()
    }

    #[must_use]
    pub fn can_promote(&self, target_role: Role) -> bool {
        match target_role {
            Role::User | Role::Moderator => self.contains(Self::MANAGE_MODERATOR),
            Role::Admin => self.contains(Self::MANAGE_ADMIN),
            Role::Owner => false, // No one can promote to Owner via this system
        }
    }
}

impl Default for Permissions {
    fn default() -> Self {
        roles::USER_ROLE
    }
}

/// Hierarchical roles within the system.
#[derive(SpacetimeType, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    User,
    Moderator,
    Admin,
    Owner,
}

impl Role {
    const fn into_permissions(self) -> Permissions {
        match self {
            Role::User => roles::USER_ROLE,
            Role::Moderator => roles::MOD_ROLE,
            Role::Admin => roles::ADMIN_ROLE,
            Role::Owner => roles::OWNER_ROLE,
        }
    }
}
