<script lang="ts">
    import { AppBar, Navigation, Popover } from '@skeletonlabs/skeleton-svelte';
    import ThemeButtons from './ThemeButtons.svelte';
    import { Info, User } from 'lucide-svelte';
    import { SignOutButton } from 'svelte-clerk';
    import { page } from '$app/state';
    import logo from '$lib/assets/wrf-logo.png';
    import type { AccountRoute } from '$lib/types/account_routes';

    const accountNavbarRoutes: AccountRoute[] = [
        {
            Icon: Info,
            href: '/app/info',
            label: 'Account Info',
            id: '0',
        },
    ];
</script>

<AppBar
    classes="sticky top-0 z-40"
    background="dark:bg-surface-900 bg-surface-500 backdrop-blur-lg !bg-opacity-70"
>
    <a href="/app" class="flex h-full w-full items-center gap-4 text-center text-3xl">
        <img src={logo} class="w-20 dark:invert" alt="wrf-logo" />
        <p class="text-white max-md:hidden">Warframe Relic Finder</p>
    </a>

    {#snippet trail()}
        <ThemeButtons />

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
                        <SignOutButton
                            class=" w-full gap-4 rounded-container px-4 py-3 text-center bg-error-200-800 hover:preset-tonal-error"
                        />
                    {/snippet}
                </Navigation.Rail>
            {/snippet}
        </Popover>
    {/snippet}
</AppBar>
