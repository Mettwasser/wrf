<script lang="ts">
    import { AppBar, Navigation, Popover, ToastProvider } from '@skeletonlabs/skeleton-svelte';
    import User from 'lucide-svelte/icons/user';
    import Info from 'lucide-svelte/icons/info';
    import LogOut from 'lucide-svelte/icons/log-out';
    import logo from '$lib/assets/wrf-logo.png';
    import { page } from '$app/state';
    import type { AccountRoute } from '$lib/types/account_routes';
    import ThemeButtons from '$lib/components/ThemeButtons.svelte';
    import { goto } from '$app/navigation';
    import { URL } from '$lib';

    let { children } = $props();

    const accountNavbarRoutes: AccountRoute[] = [
        {
            Icon: Info,
            href: '/app/info',
            label: 'Account Info',
            id: '0',
        },
    ];

    async function logout() {
        let resp = await fetch(`${URL}/api/auth/logout`, {
            method: 'POST',
            credentials: 'include',
        });
        if (resp.ok) goto('/login');
    }

    console.log('Layout rendered');
</script>

<div class="flex size-full flex-col">
    <AppBar
        classes="sticky top-0 z-40"
        background="dark:bg-surface-900 bg-surface-500 backdrop-blur-lg !bg-opacity-70"
    >
        <a href="/app" class="flex h-full w-full items-center gap-4 text-center text-3xl">
            <img src={logo} class="w-20 dark:invert" alt="wrf-logo" />
            Warframe Relic Finder
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
                            <Navigation.Tile
                                background="bg-error-200-800"
                                hover="hover:preset-tonal-error"
                                labelExpanded="Logout"
                                onclick={logout}
                                id="logout"
                            >
                                <LogOut />
                            </Navigation.Tile>
                        {/snippet}
                    </Navigation.Rail>
                {/snippet}
            </Popover>
        {/snippet}
    </AppBar>

    <ToastProvider placement="bottom-end">
        <div class="flex flex-1 p-4">
            {@render children()}
        </div>
    </ToastProvider>
</div>
