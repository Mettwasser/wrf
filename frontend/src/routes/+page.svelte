<script lang="ts">
    import { fade, fly, slide } from 'svelte/transition';
    import { cubicOut } from 'svelte/easing';
    import {
        Zap,
        Users,
        Target,
        ShieldCheck,
        ArrowRight,
        Sparkles,
        MessageSquareX,
        MessageSquareCheck,
        Search,
    } from 'lucide-svelte';
    import meme from '$lib/assets/meme.jpg';
    import logo from '$lib/assets/wrf-logo.png';
    import { SiDiscord } from '@icons-pack/svelte-simple-icons';

    let mounted = $state(false);
    $effect(() => {
        mounted = true;
    });

    const features = [
        {
            icon: Zap,
            title: 'Matchmaking',
            description: 'No more copy-pasting into recruit chat. Find or host lobbies in seconds.',
            color: 'text-primary-500',
        },
        {
            icon: ShieldCheck,
            title: 'Verification System',
            description: 'An optional verification guardrail for when trolls get out of hand.',
            color: 'text-success-600',
        },
        {
            icon: Search,
            title: 'Filtered',
            description:
                'Radshares, Intshares, or anything else - filter exactly for what you need.',
            color: 'text-secondary-500',
        },
    ];
</script>

<svelte:head>
    <style>
        /* Global overflow prevention for this page */
        html,
        body {
            overflow-x: hidden;
            width: 100%;
            position: relative;
        }
    </style>
</svelte:head>

<div class="bg-surface-950 text-surface-50 relative min-h-screen w-full">
    <!-- BG Elements -->
    <div class="pointer-events-none fixed inset-0 z-0 overflow-hidden select-none">
        <div
            class="bg-primary-500/10 absolute -top-16 -left-16 size-48 animate-pulse rounded-full opacity-70 blur-[60px] sm:-top-24 sm:-left-24 sm:size-96 sm:blur-[100px]"
        ></div>
        <div
            class="bg-secondary-500/10 absolute top-1/2 -right-16 size-48 animate-pulse rounded-full opacity-70 blur-[60px] [animation-delay:2s] sm:-right-24 sm:size-96 sm:blur-[100px]"
        ></div>
        <div
            class="bg-tertiary-500/10 absolute -bottom-16 left-1/4 size-48 animate-pulse rounded-full opacity-70 blur-[60px] [animation-delay:4s] sm:-bottom-24 sm:size-96 sm:blur-[100px]"
        ></div>
    </div>

    <!-- Main Content Wrapper -->
    <div class="relative z-10 w-full overflow-x-hidden">
        <!-- Hero Section -->
        <header class="flex flex-col items-center px-4 pt-16 pb-12 text-center sm:pt-32 sm:pb-20">
            {#if mounted}
                <div
                    in:fly={{ y: -20, duration: 800, easing: cubicOut }}
                    class="mb-4 flex items-center gap-3 sm:mb-6 sm:gap-4"
                >
                    <img
                        src={logo}
                        alt="WRF Logo"
                        class="w-10 drop-shadow-[0_0_15px_rgba(var(--color-primary-500),0.5)] sm:w-16"
                        style="filter: invert(1) brightness(2);"
                    />
                    <span
                        class="text-surface-400 text-xs font-bold tracking-widest uppercase sm:text-xl"
                    >
                        WRF
                    </span>
                </div>

                <h1
                    in:fly={{ y: 20, duration: 1000, delay: 200, easing: cubicOut }}
                    class="max-w-4xl text-3xl font-black tracking-tighter text-balance sm:text-7xl lg:text-8xl"
                >
                    STOP WAITING, <br />
                    <span
                        class="from-primary-400 via-secondary-300 to-tertiary-400 bg-linear-to-r bg-clip-text text-transparent"
                    >
                        START FARMING
                    </span>
                </h1>

                <p
                    in:fly={{ y: 20, duration: 1000, delay: 400, easing: cubicOut }}
                    class="text-surface-400 mt-6 max-w-2xl text-sm text-balance sm:mt-8 sm:text-xl"
                >
                    The ultimate coordination hub for Warframe Relic hunters. Skip the recruit chat
                    chaos and find your squad instantly.
                </p>

                <div
                    in:fly={{ y: 20, duration: 1000, delay: 600, easing: cubicOut }}
                    class="mt-8 flex w-full flex-col items-center justify-center gap-4 sm:mt-12 sm:w-auto sm:flex-row"
                >
                    <a
                        href="/app"
                        class="btn btn-lg preset-filled-primary-500 group flex w-full items-center justify-center gap-2 px-6 py-4 text-base font-bold transition-all hover:scale-105 hover:shadow-[0_0_20px_rgba(var(--color-primary-500),0.4)] sm:w-auto sm:px-8 sm:text-lg"
                    >
                        ENTER THE HUB
                        <ArrowRight class="transition-transform group-hover:translate-x-1" />
                    </a>
                    <a
                        href="#the-struggle"
                        class="btn btn-lg preset-outlined-surface-500 hover:bg-surface-50/10 w-full px-6 py-4 text-base font-bold transition-all sm:w-auto sm:px-8 sm:text-lg"
                    >
                        LEARN MORE
                    </a>
                </div>
            {/if}
        </header>

        <!-- The Struggle Section -->
        <section id="the-struggle" class="mx-auto max-w-6xl px-4 py-12 sm:py-24">
            <div class="grid items-center gap-8 lg:grid-cols-2 lg:gap-12">
                <div class=" flex flex-col gap-8 sm:gap-12">
                    <div class="flex flex-col gap-4 sm:gap-6">
                        <div class="text-error-500 flex items-center gap-3">
                            <MessageSquareX size={24} class="sm:size-8" />
                            <span class="text-xs font-bold tracking-[0.2em] uppercase sm:text-sm">
                                The Old Way
                            </span>
                        </div>
                        <h2 class="text-2xl font-bold tracking-tight sm:text-4xl md:text-5xl">
                            Recruit chat is <br />
                            where the
                            <span class="text-error-500 font-black">profit dies</span>
                        </h2>
                        <p class="text-surface-400 text-sm sm:text-lg">
                            Spending 30 minutes spamming
                            <code class="code">H [Lith N13 Relic] rad 1/4</code>
                            only to have the party disband after one run.
                        </p>
                    </div>
                </div>

                <div>
                    <img
                        src={meme}
                        alt="Recruit chat struggle"
                        class="border-surface-200 w-full rounded-xl border-2 p-1 opacity-90 grayscale-[0.2] transition-all"
                    />
                </div>
            </div>
        </section>

        <!-- The Solution -->
        <section id="the-solution" class="mx-auto max-w-6xl px-4 py-12 sm:py-24">
            <div class="grid items-center gap-8 lg:grid-cols-2 lg:gap-12">
                <div class="max-lg:order-2">
                    <img
                        src={logo}
                        alt="Recruit chat struggle"
                        class="border-surface-200 w-full rounded-xl border-2 p-4 opacity-80 grayscale-[0.2] invert transition-all"
                    />
                </div>
                <div class="flex flex-col gap-8 sm:gap-12">
                    <div class="flex flex-col gap-4 pt-8 sm:gap-6">
                        <div class="text-success-500 flex items-center gap-3">
                            <MessageSquareCheck size={24} class="sm:size-8" />
                            <span class="text-xs font-bold tracking-[0.2em] uppercase sm:text-sm">
                                The WRF Way
                            </span>
                        </div>
                        <h2 class="text-2xl font-bold tracking-tight sm:text-4xl md:text-5xl">
                            Find your squad <br />
                            in
                            <span class="text-success-500 font-black">seconds</span>
                        </h2>
                        <p class="text-surface-300 text-sm font-bold sm:text-lg">
                            An easy-to-use dashboard to find or host exactly what you need. Built by
                            Tenno, for Tenno, to make the grind actually enjoyable.
                        </p>
                    </div>
                </div>
            </div>
        </section>

        <!-- Features Grid -->
        <section class="bg-surface-900/30 relative z-10 py-12 backdrop-blur-sm sm:py-24">
            <div class="mx-auto max-w-6xl px-4">
                <div class="mb-8 text-center sm:mb-16">
                    <h2 class="text-xl font-bold sm:text-4xl">
                        Developed to make <br />
                        your life
                        <span class="text-primary-400">easier</span>
                    </h2>
                    <p class="text-surface-400 mt-2 text-sm sm:mt-4 sm:text-base">
                        Everything you need to optimize your prime part farming.
                    </p>
                </div>

                <div class="grid gap-4 sm:gap-8 md:grid-cols-3">
                    {#each features as feature, i (i)}
                        <div
                            class="card group border-surface-700 bg-surface-800/40 hover:border-primary-500/50 cursor-default border p-5 transition-all hover:-translate-y-2 sm:p-8"
                        >
                            <div
                                class="bg-surface-950 group-hover:bg-primary-500/10 mb-4 flex size-10 items-center justify-center rounded-xl shadow-inner sm:mb-6 sm:size-14"
                            >
                                <feature.icon class="{feature.color} size-6 sm:size-7" />
                            </div>
                            <h3 class="mb-2 text-base font-bold sm:mb-4 sm:text-xl">
                                {feature.title}
                            </h3>
                            <p class="text-surface-400 text-xs leading-relaxed sm:text-base">
                                {feature.description}
                            </p>
                        </div>
                    {/each}
                </div>
            </div>
        </section>

        <!-- Final CTA -->
        <section class="px-4 py-16 text-center sm:py-32">
            <div
                class="border-surface-700 from-surface-900 to-surface-950 mx-auto max-w-3xl rounded-3xl border bg-linear-to-br p-6 shadow-[0_0_50px_rgba(0,0,0,0.5)] sm:p-12"
            >
                <Sparkles class="text-gold-400 mx-auto mb-4 sm:mb-6" size={32} />
                <h2 class="mb-4 text-xl font-bold sm:mb-6 sm:text-4xl">
                    Ready to find your squad?
                </h2>
                <p class="text-surface-400 mb-8 text-sm sm:mb-10 sm:text-xl">
                    Don't grind alone - join the other Tenno already using WRF to hunt Prime Parts.
                </p>
                <a
                    href="/app"
                    class="btn btn-xl preset-filled-primary-500 w-full px-8 py-4 text-base font-black tracking-tight transition-all sm:w-auto sm:px-12 sm:py-5 sm:text-xl"
                >
                    LAUNCH THE APP
                </a>
            </div>
        </section>

        <!-- Footer -->
        <footer class="border-surface-800 text-surface-500 border-t py-8 text-center sm:py-12">
            <p class="px-4 text-[10px] sm:text-base">
                © 2026 Warframe Relic Finder. Not affiliated with Digital Extremes.
            </p>
            <div class="mt-4 flex justify-center gap-6">
                <a
                    href="https://discord.gg/VANqqkv8pp"
                    target="_blank"
                    class="text-surface-100 flex gap-2"
                >
                    <SiDiscord /> Discord
                </a>
            </div>
        </footer>
    </div>
</div>

<style>
    :global(body) {
        background-color: #0a0a0a;
        height: auto !important;
    }
</style>
