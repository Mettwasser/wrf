<script lang="ts">
    import '../app.css';
    import { updateDarkModeInHtml } from '$lib/utils/dark_mode';
    import { darkModeState, theme } from '$lib';
    import { updateThemeInHtml } from '$lib/utils/theme';

    let { children } = $props();

    updateDarkModeInHtml(darkModeState.current);
    updateThemeInHtml(theme.current as any);
</script>

<!-- Height is set to auto to resolve a weird issue with the navbar being not fixed after scrolling for a while -->
<div class="flex flex-1">
    {@render children()}
</div>
