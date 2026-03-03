import type { Component, SvelteComponent } from 'svelte';

export interface AccountRoute {
    Icon: typeof SvelteComponent;
    id: string;
    label: string;
    href: string;
}
