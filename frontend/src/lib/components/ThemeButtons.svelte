<script lang="ts">
    import SunMoon from 'lucide-svelte/icons/sun-moon';

    import Palette from 'lucide-svelte/icons/palette';
    import { THEMES, updateThemeInHtml } from '$lib/utils/theme';
    import { updateDarkModeInHtml, type DarkModeState } from '$lib/utils/darkMode';
    import type { ChangeEventHandler } from 'svelte/elements';
    import { darkModeState, theme } from '$lib';

    function toggleDarkMode() {
        if (darkModeState.current === 'dark') darkModeState.current = 'light';
        else darkModeState.current = 'dark';
        updateDarkModeInHtml(darkModeState.current);
    }

    const switchTheme: ChangeEventHandler<HTMLSelectElement> = (e) => {
        theme.current = THEMES.find((theme) => theme.name === e.currentTarget!.value)!;
        updateThemeInHtml(theme.current as any);
    };
</script>

<div class="input-group hover:preset-tonal xsm:flex hidden grid-cols-[auto_1fr]">
    <div class="input-group-cell">
        <Palette size={18} />
    </div>
    <select
        class="select !outline-surface-950 capitalize"
        bind:value={theme.current.name}
        onchange={switchTheme}
    >
        {#each THEMES as theme}
            <option value={theme.name} class="!border-none capitalize">{theme.name}</option>
        {/each}
    </select>
</div>

<div class="flex items-center justify-center">
    <button type="button" onclick={toggleDarkMode} class="hover:preset-tonal rounded-full p-2">
        <SunMoon size={30} />
    </button>
</div>
