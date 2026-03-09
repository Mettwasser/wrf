<script lang="ts">
    import '../app.css';

    import { theme } from '$lib';
    import { updateThemeInHtml } from '$lib/utils/theme';
    import { ClerkProvider } from 'svelte-clerk';

    let { children } = $props();

    updateThemeInHtml(theme.current);

    const publishableKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
</script>

<ClerkProvider {publishableKey}>
    <div class="flex h-full flex-1">
        {@render children()}
    </div>
</ClerkProvider>
