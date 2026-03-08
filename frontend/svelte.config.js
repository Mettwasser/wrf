import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),

    kit: {
        adapter: adapter({
            fallback: 'index.html',
        }),
    },
    compilerOptions: {
        warningFilter: (w) =>
            !['a11y_consider_explicit_label', 'state_referenced_locally'].includes(w.code),
    },
};

export default config;
