<script lang="ts">
    import '../app.css';
    import { updateDarkModeInHtml } from '$lib/utils/darkMode';
    import { darkModeState, theme } from '$lib';
    import { updateThemeInHtml } from '$lib/utils/theme';

    let { children } = $props();

    updateDarkModeInHtml(darkModeState.current);
    updateThemeInHtml(theme.current as any);
</script>

<div class="flex size-full">
    {@render children()}
</div>
