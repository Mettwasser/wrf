// place files you want to import through the `$lib` alias in this folder.

import { dev } from '$app/environment';
import { PersistedState } from 'runed';
import type { User } from './types/user';
import type { Theme } from '@skeletonlabs/skeleton/themes';
import * as themes from '@skeletonlabs/skeleton/themes';
import type { DarkModeState } from './utils/darkMode';

export enum NavbarDisplayOptions {
    Hidden,
    ThemesOnly,
    Full,
}

export const URL = dev ? 'http://localhost:5150' : 'TODO';

export const theme = new PersistedState<Theme>('theme', themes.cerberus);
export const darkModeState = new PersistedState<DarkModeState>('darkModeState', 'dark');

export const dateReviver = (k: any, v: any) => {
    if (typeof v === 'string') {
        let d = new Date(v);
        if (Number.isNaN(d.getTime())) return v;
        return d;
    }
    return v;
};
