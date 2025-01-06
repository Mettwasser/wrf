import { dateReviver, URL } from '$lib/index.svelte';
import type { RegisterSession } from '$lib/types/register_session.js';
import { error } from '@sveltejs/kit';

export const load = ({ fetch }) => {
    const wrap = async () => {
        const response = await fetch(`${URL}/api/register_sessions/current`, {
            credentials: 'include',
        });
        let json = await response.text();
        return JSON.parse(json, dateReviver) as RegisterSession;
    };

    return { response: wrap() };
};
