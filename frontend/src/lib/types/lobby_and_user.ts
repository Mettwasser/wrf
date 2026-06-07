import type { Lobby, User } from '$lib/module_bindings/types';
import type { FullUser } from './full_user';

export interface LobbyAndUser {
    user: FullUser;
    lobby: Lobby;
}
