import containerQueries from '@tailwindcss/container-queries';
import forms from '@tailwindcss/forms';
import type { Config } from 'tailwindcss';

import { skeleton, contentPath } from '@skeletonlabs/skeleton/plugin';
import * as themes from '@skeletonlabs/skeleton/themes';

export default {
    content: ['./src/**/*.{html,js,svelte,ts}', contentPath(import.meta.url, 'svelte')],

    theme: {
        extend: {
            screens: {
                xsm: '680px',
            },
        },
    },

    darkMode: 'selector',

    plugins: [
        containerQueries,
        forms,
        skeleton({
            themes: Object.values(themes),
        }),
    ],
} satisfies Config;
