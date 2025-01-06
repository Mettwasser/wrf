<script lang="ts">
    import '../app.css';
    import { THEMES, updateThemeInHtml } from '$lib/utils/theme';
    import * as themes from '@skeletonlabs/skeleton/themes';
    import { AppBar, Navigation, Popover, ToastProvider } from '@skeletonlabs/skeleton-svelte';
    import SunMoon from 'lucide-svelte/icons/sun-moon';
    import Palette from 'lucide-svelte/icons/palette';
    import User from 'lucide-svelte/icons/user';
    import Info from 'lucide-svelte/icons/info';
    import LogOut from 'lucide-svelte/icons/log-out';
    import { PersistedState } from 'runed';
    import type { Theme } from '@skeletonlabs/skeleton/themes';
    import { updateDarkModeInHtml, type DarkModeState } from '$lib/utils/darkMode';
    import type { ChangeEventHandler } from 'svelte/elements';
    import logo from '$lib/assets/wrf-logo.png';
    import type { Component } from 'svelte';
    import { page } from '$app/state';
    import { NavbarDisplayOptions, navbarOpts } from '$lib/index.svelte';

    let { children } = $props();

    let theme = new PersistedState<Theme>('theme', themes.cerberus);
    updateThemeInHtml(theme.current as any);
    let darkModeState = new PersistedState<DarkModeState>('darkModeState', 'dark');

    function toggleDarkMode() {
        if (darkModeState.current === 'dark') darkModeState.current = 'light';
        else darkModeState.current = 'dark';
        updateDarkModeInHtml(darkModeState.current);
    }

    const switchTheme: ChangeEventHandler<HTMLSelectElement> = (e) => {
        theme.current = THEMES.find((theme) => theme.name === e.currentTarget!.value)!;
        updateThemeInHtml(theme.current as any);
    };

    const accountNavbarRoutes: {
        Icon: Component;
        id: string;
        label: string;
        href: string;
    }[] = [
        {
            href: '/',
            label: 'Account Info',
            Icon: Info as any,
            id: '0',
        },
    ];

    // let navbarShouldHide = $derived(['/register'].includes(page.url.pathname));
</script>

{#snippet themeButtons()}
    <div class="input-group hover:preset-tonal xsm:flex hidden grid-cols-[auto_1fr]">
        <div class="input-group-cell">
            <Palette size={18} />
        </div>
        <select class="capitalize" bind:value={theme.current.name} onchange={switchTheme}>
            {#each THEMES as theme}
                <option value={theme.name} class="capitalize">{theme.name}</option>
            {/each}
        </select>
    </div>

    <div class="flex items-center justify-center">
        <button type="button" onclick={toggleDarkMode} class="hover:preset-tonal rounded-full p-2">
            <SunMoon size={30} />
        </button>
    </div>
{/snippet}

<div class="flex size-full flex-col">
    {#if navbarOpts.display === NavbarDisplayOptions.ThemesOnly}
        <div class="absolute flex w-full justify-end gap-4 p-4">
            {@render themeButtons()}
        </div>
    {:else if navbarOpts.display === NavbarDisplayOptions.Full}
        <AppBar
            classes="sticky top-0 z-40"
            background="dark:bg-surface-900 bg-surface-500 backdrop-blur-lg !bg-opacity-70"
        >
            {#snippet trail()}
                {@render themeButtons()}

                <Popover
                    positionerZIndex="!z-50"
                    arrow
                    contentBase="card bg-surface-200-800 p-2"
                    triggerBase="hover:preset-tonal rounded-full p-2 "
                    base="flex justify-center items-center"
                    arrowBackground="!bg-surface-200 dark:!bg-surface-800"
                >
                    {#snippet trigger()}
                        <User size={28} />
                    {/snippet}
                    {#snippet content()}
                        <Navigation.Rail expanded background="bg-transparent">
                            {#snippet tiles()}
                                {#each accountNavbarRoutes as { href, Icon, id, label }}
                                    <Navigation.Tile
                                        {id}
                                        labelExpanded={label}
                                        {href}
                                        selected={page.url.pathname === href}
                                    >
                                        <Icon />
                                    </Navigation.Tile>
                                {/each}
                                <Navigation.Tile
                                    background="bg-error-200-800"
                                    hover="hover:preset-tonal-error"
                                    labelExpanded="Logout"
                                    active="active:!preset-filled-error-300-700"
                                >
                                    <LogOut />
                                </Navigation.Tile>
                            {/snippet}
                        </Navigation.Rail>
                    {/snippet}
                </Popover>
            {/snippet}

            <div class="flex h-full w-full items-center gap-4 text-center text-3xl">
                <img src={logo} class="w-20 dark:invert" alt="wrf-logo" />
                Warframe Relic Finder
            </div>
        </AppBar>
    {/if}
    <ToastProvider placement="bottom-end">
        <div class="flex flex-1 p-4">
            {@render children()}
        </div>
    </ToastProvider>
</div>
