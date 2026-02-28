<script lang="ts">
    import Palette from 'lucide-svelte/icons/palette';
    import { THEMES, updateThemeInHtml } from '$lib/utils/theme';
    import type { ChangeEventHandler } from 'svelte/elements';
    import { theme } from '$lib';

    const switchTheme: ChangeEventHandler<HTMLSelectElement> = (e) => {
        theme.current = e.currentTarget!.value;
        updateThemeInHtml(theme.current);
    };
</script>

<div class="input-group hover:preset-tonal grid-cols-[auto_1fr_auto]">
    <div class="ig-cell">
        <Palette size={18} />
    </div>
    <select class="ig-select capitalize ring-0" bind:value={theme.current} onchange={switchTheme}>
        {#each THEMES as t}
            <option value={t} class="border-none! capitalize">{t}</option>
        {/each}
    </select>
</div>
