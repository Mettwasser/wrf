import { goto } from '$app/navigation';
import { URL } from '$lib';
import type { Relic } from '$lib/types/relic.js';
import type { User } from '$lib/types/user';

async function fetchAuth(fetch: typeof window.fetch) {
    const response = await fetch(`${URL}/api/auth/current`, {
        credentials: 'include',
    });

    if (response.status === 401) goto('/login');

    let json = await response.text();

    return JSON.parse(json) as User;
}

type InitialRelicData = {
    [key: string]: Relic;
};

async function fetchRelics(fetch: typeof window.fetch) {
    const response = await fetch(
        'https://raw.githubusercontent.com/calamity-inc/warframe-public-export-plus/refs/heads/senpai/ExportRelics.json'
    );

    let json = await response.text();

    let data: InitialRelicData = JSON.parse(json);

    // copium
    let mapped: string[] = Object.values(data).map((relic) => {
        return JSON.stringify({
            category: relic.category,
            era: relic.era,
        });
    });

    return [...new Set(mapped)].map((s) => JSON.parse(s)) as Relic[];
}

export const load = async ({ fetch }) => {
    const [user, relics] = await Promise.all([fetchAuth(fetch), fetchRelics(fetch)]);

    return { user, relics };
};
