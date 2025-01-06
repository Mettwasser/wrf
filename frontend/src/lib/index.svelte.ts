// place files you want to import through the `$lib` alias in this folder.

import { dev } from '$app/environment';

export enum NavbarDisplayOptions {
    Hidden,
    ThemesOnly,
    Full,
}

export const navbarOpts = $state({ display: NavbarDisplayOptions.Full });

export const URL = dev ? 'http://localhost:5150' : 'TODO';

export const dateReviver = (k: any, v: any) => {
    if (typeof v === 'string') {
        let d = new Date(v);
        if (Number.isNaN(d.getTime())) return v;
        return d;
    }
    return v;
};
