import type { Lobby, User } from '$lib/module_bindings/types';

export interface LobbyAndUser {
    user: User;
    lobby: Lobby;
}
