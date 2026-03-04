import type { Component, SvelteComponent } from 'svelte';

export interface NavbarRoute {
    Icon: typeof SvelteComponent<any>;
    id: string;
    label: string;
    href: string;
}
