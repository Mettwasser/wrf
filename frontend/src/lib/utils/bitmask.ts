export enum UserFlags {
    None = 0,
    Verified = 1 << 0,
    Banned = 1 << 1,
}

export enum Permissions {
    None = 0,
    CREATE_LOBBY = 1 << 0,
    JOIN_LOBBY = 1 << 1,
    BAN_USERS = 1 << 2,
    MANAGE_MODERATOR = 1 << 3,
    MANAGE_ADMIN = 1 << 4,
}

export const Bitmask = {
    has: (userPerms: number, required: number): boolean => {
        return (userPerms & required) === required;
    },
};
