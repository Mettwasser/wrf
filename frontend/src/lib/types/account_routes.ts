import type { Component, SvelteComponentTyped } from 'svelte';

export interface AccountRoute<
    Props extends Record<string, any> = Record<string, any>,
    Events extends Record<string, any> = any,
    Slots extends Record<string, any> = any,
> {
    Icon: typeof SvelteComponentTyped<Props, Events, Slots>;
    id: string;
    label: string;
    href: string;
}
