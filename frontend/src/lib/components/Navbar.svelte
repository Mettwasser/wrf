<script lang="ts">
    import { AppBar, Popover } from '@skeletonlabs/skeleton-svelte';
    import ThemeButtons from './ThemeButtons.svelte';
    import { House, Info, LogOut, User } from 'lucide-svelte';
    import { page } from '$app/state';
    import logo from '$lib/assets/wrf-logo.png';
    import type { NavbarRoute } from '$lib/types/account_routes';
    import { SiDiscord } from '@icons-pack/svelte-simple-icons';
    import { SignOutButton, UserAvatar } from 'svelte-clerk';

    const links: NavbarRoute[] = [
        { label: 'Home', href: '/app', Icon: House, id: '0' },
        {
            label: 'Discord',
            href: 'https://discord.gg/VANqqkv8pp',
            Icon: SiDiscord,
            id: '1',
            openInNewTab: true,
        },
    ];

    const accountNavbarRoutes: NavbarRoute[] = [
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
        <AppBar.Lead class="max-xsm:hidden flex items-center ">
            <a href="/app" class="h4 text-surface-50! flex size-full items-center gap-4">
                <img src={logo} class="w-20 dark:invert" alt="wrf-logo" />
                <span class="hidden text-nowrap sm:block">Warframe Relic Finder</span>
            </a>
        </AppBar.Lead>

        <AppBar.Headline class="flex h-full items-center justify-center">
            {#each links as { Icon, href, id, label, openInNewTab } (id)}
                <a
                    {href}
                    class="hover:preset-tonal *:text-surface-50 card flex h-full items-center justify-center gap-2 px-4"
                    target={openInNewTab ? '_blank' : undefined}
                >
                    <Icon class=" size-6 h-full" />
                    <span class="font-bold">{label}</span>
                </a>
            {/each}
        </AppBar.Headline>

        <AppBar.Trail class="flex justify-end">
            <ThemeButtons />

            <Popover positioning={{ placement: 'bottom-end' }}>
                <Popover.Trigger class="hover:preset-tonal mx-1 cursor-pointer rounded-full p-1">
                    <UserAvatar />
                </Popover.Trigger>
                <Popover.Positioner>
                    <Popover.Content class="card bg-surface-200-800 xsm:w-80 w-72 p-2">
                        <div class="flex flex-col gap-2">
                            {#each accountNavbarRoutes as { href, Icon, id, label }}
                                <a
                                    {id}
                                    {href}
                                    class="hover:preset-tonal text-surface-50! flex items-center gap-2 rounded-sm p-3 {page
                                        .url.pathname === href
                                        ? 'preset-tonal'
                                        : ''}"
                                >
                                    <Icon />
                                    {label}
                                </a>
                            {/each}
                            <SignOutButton asChild redirectUrl="/login">
                                {#snippet children({ signOut }: { signOut: any })}
                                    <button
                                        onclick={signOut}
                                        class="text-error-500 hover:bg-error-500/40 hover:text-surface-50 flex w-full items-center gap-2 rounded-sm p-3"
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

<style>
    :global(.cl-avatarBox) {
        height: 2.25rem;
        width: 2.25rem;
    }
</style>
