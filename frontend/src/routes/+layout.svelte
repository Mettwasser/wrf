<script lang="ts">
    import '../app.css';
    import { updateDarkModeInHtml } from '$lib/utils/dark_mode';
    import { darkModeState, theme } from '$lib';
    import { updateThemeInHtml } from '$lib/utils/theme';
    import { ClerkProvider } from 'svelte-clerk';

    let { children } = $props();

    updateDarkModeInHtml(darkModeState.current);
    updateThemeInHtml(theme.current as any);

    const publishableKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
</script>

<ClerkProvider {publishableKey}>
    <div class="flex flex-1">
        {@render children()}
    </div>
</ClerkProvider>
