import { PersistedState } from 'runed';
import { getContext } from 'svelte';
import { DbConnection } from './module_bindings';
import type { Identity } from 'spacetimedb';

import { createToaster } from '@skeletonlabs/skeleton-svelte';

export const theme = new PersistedState<string>('theme', 'cerberus');

export const toaster = createToaster({
    placement: 'bottom',
    overlap: true,
});

export function makeToComboboxData(collection: string[]): ComboboxData[] {
    return collection.map((item) => {
        return { label: item, value: item };
    });
}

export function conn(): DbConnection {
    // @ts-ignore
    return getContext('conn').current;
}

export function identity(): Identity {
    // @ts-ignore
    return getContext('ident').current;
}

export const useId = (() => {
    let id = 0;
    return () => Math.random().toString(36).substring(2) + id++;
})();
