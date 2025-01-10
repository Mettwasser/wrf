import { goto } from '$app/navigation';
import { URL } from '$lib';
import type { User } from '$lib/types/user';

export const load = async ({ fetch }) => {
    const response = await fetch(`${URL}/api/auth/current`, {
        credentials: 'include',
    });

    if (response.status === 401) goto('/login');

    let json = await response.text();

    return { user: JSON.parse(json) as User };
};
