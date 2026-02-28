<script lang="ts">
    import { AppBar, Popover } from '@skeletonlabs/skeleton-svelte';
    import ThemeButtons from './ThemeButtons.svelte';
    import { Info, LogOut, User } from 'lucide-svelte';
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
    class="dark:bg-surface-900 bg-surface-500 bg-opacity-70! sticky top-0 z-40 backdrop-blur-lg"
>
    <AppBar.Toolbar class="grid-cols-[auto_1fr_auto]">
        <AppBar.Lead>
            <a href="/app" class="flex h-full w-full items-center gap-4 text-center text-3xl">
                <img src={logo} class="w-20 dark:invert" alt="wrf-logo" />
            </a>
        </AppBar.Lead>
        <AppBar.Headline>
            <a href="/app" class="h4 text-white! max-md:hidden">Warframe Relic Finder</a>
        </AppBar.Headline>

        <AppBar.Trail>
            <ThemeButtons />

            <Popover positioning={{ placement: 'bottom-end' }}>
                <Popover.Trigger class="hover:preset-tonal cursor-pointer rounded-full p-2">
                    <User size={28} />
                </Popover.Trigger>
                <Popover.Positioner>
                    <Popover.Content class="card bg-surface-200-800 w-80 p-2">
                        <div class="flex flex-col gap-2">
                            {#each accountNavbarRoutes as { href, Icon, id, label }}
                                <a
                                    {id}
                                    {href}
                                    class="hover:preset-tonal flex items-center gap-2 rounded-sm p-3 text-white! {page
                                        .url.pathname === href
                                        ? 'preset-tonal'
                                        : ''}"
                                >
                                    <Icon />
                                    {label}
                                </a>
                            {/each}
                            <SignOutButton asChild>
                                {#snippet children({ signOut })}
                                    <button
                                        onclick={signOut}
                                        class="text-error-500 hover:bg-error-500/40 flex w-full items-center gap-2 rounded-sm p-3 hover:text-white"
                                        id="logoutbtn"
                                    >
                                        <LogOut />
                                        <span>Sign Out</span>
                                    </button>
                                {/snippet}
                            </SignOutButton>
                        </div>

                        <Popover.Arrow
                            class="[--arrow-background:var(--color-surface-200-800)] [--arrow-size:--spacing(2)]"
                        >
                            <Popover.ArrowTip />
                        </Popover.Arrow>
                    </Popover.Content>
                </Popover.Positioner>
            </Popover>
        </AppBar.Trail>
    </AppBar.Toolbar>
</AppBar>
