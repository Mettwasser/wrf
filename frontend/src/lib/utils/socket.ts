import { URL } from '$lib';
import { io as ioClient } from 'socket.io-client';

const socket = ioClient(`${URL}`, {
    withCredentials: true,
});

export const io = socket;

// Define the SubscriptionType discriminated union
type SubscriptionType = { type: 'Recent' } | { type: 'Lobby'; id: number };

enum ClientEvent {
    Subscribe = 'subscribe',
    Unsubscribe = 'unsubscribe',
}

enum ServerEvent {
    CreateLobby = 'create-lobby',
    PlayerCountUpdate = 'player-count-update',
}

export { type SubscriptionType, ServerEvent, ClientEvent };
