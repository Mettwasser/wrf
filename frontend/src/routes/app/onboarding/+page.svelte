<script lang="ts">
    import { goto } from '$app/navigation';
    import { tables } from '$lib/module_bindings';
    import { useTable } from 'spacetimedb/svelte';
    import { fly } from 'svelte/transition';
    import wrfLogo from '$lib/assets/wrf-logo.png';
    import { UserRound, ShieldCheck, ChevronRight, LoaderCircle } from 'lucide-svelte';

    const [me, meIsReady] = useTable(tables.me);

    let show = $state(false);

    $effect(() => {
        if ($meIsReady) {
            const user = $me[0];
            if (user && user.username !== '') {
                goto('/app');
            } else {
                show = true;
            }
        }
    });
</script>

{#if !$meIsReady}
    <LoaderCircle class="size-6 animate-spin" />
{:else if show}
    <div class="flex size-full flex-col items-center justify-center p-4">
        <div
            in:fly={{ y: 20, duration: 600 }}
            class="card bg-surface-200-800/40 shadow-surface-100/10 preset-outlined-surface-600-400 flex max-w-xl flex-col gap-8 border-2! p-8 shadow-lg backdrop-blur-lg md:p-12"
        >
            <div class="flex items-center gap-4">
                <img src={wrfLogo} alt="WRF" class="w-12 invert" />
                <h1 class="h2 font-bold tracking-tight">WRF</h1>
            </div>

            <div class="space-y-4">
                <p class="text-surface-100">
                    To get started, you just need to do a couple of things:
                </p>

                <div class="grid gap-4">
                    <div
                        class="card bg-surface-500/5 preset-outlined-surface-300-700 flex items-start gap-4 p-4 shadow-sm"
                    >
                        <div
                            class="preset-filled-primary-500 mt-1 flex size-8 shrink-0 items-center justify-center rounded"
                        >
                            <UserRound size={18} />
                        </div>
                        <div>
                            <p class="font-bold">1. Set your IGN</p>
                            <p class="text-surface-200 text-sm">
                                Enter your in-game username so people know who to invite.
                            </p>
                        </div>
                    </div>

                    <div
                        class="card bg-surface-500/5 preset-outlined-surface-300-700 flex items-start gap-4 p-4 shadow-sm"
                    >
                        <div
                            class="preset-filled-surface-500 mt-1 flex size-8 shrink-0 items-center justify-center rounded"
                        >
                            <ShieldCheck size={18} />
                        </div>
                        <div>
                            <p class="font-bold">2. Verification (Optional)</p>
                            <p class="text-surface-200 text-sm">
                                Verify your account if you want a badge, but you don't have to.
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex flex-col gap-4 pt-4">
                <a
                    href="/app/info?from=onboarding"
                    class="btn preset-filled-primary-500 flex items-center justify-center gap-2 py-3 text-lg font-bold"
                >
                    Set up my Profile
                    <ChevronRight size={20} />
                </a>
            </div>
        </div>
    </div>
{/if}
