<script lang="ts">
    import { MapPin, Layers, LoaderCircle, Trash2, LogOut, LogIn } from 'lucide-svelte';
    import { type Lobby } from '$lib/module_bindings/types';
    import { RotationType } from '$lib/module_bindings/types';

    interface Props {
        lobby: Lobby;
        relicUrl: string;
        refinementTextColor: string;
        isHost: boolean;
        isJoined: boolean;
        lobbyButtonLoading: boolean;
        onLeave: () => void;
        onJoin: () => void;
    }

    let {
        lobby,
        relicUrl,
        refinementTextColor,
        isHost,
        isJoined,
        lobbyButtonLoading,
        onLeave,
        onJoin,
    }: Props = $props();

    const is2A2B = $derived(lobby.rotationType.tag === RotationType.TwoATwoB.tag);
</script>

<div class="z-10 flex flex-col lg:col-span-1">
    <div
        class="card bg-surface-200-800/40 shadow-surface-100/10 preset-outlined-surface-600-400 flex h-full flex-col p-8 shadow-xl backdrop-blur-lg lg:rounded-tr-none lg:rounded-br-none lg:shadow-none"
    >
        <div class="flex flex-col items-center gap-6">
            <img src={relicUrl} alt="Relic" class="h-48 w-48 object-contain drop-shadow-2xl" />

            <div class="text-center">
                <h2 class="h2 mb-2 leading-tight font-bold">{lobby.activity}</h2>
                <p class="text-3xl font-black {refinementTextColor}">
                    {lobby.refinement.tag}
                </p>
            </div>

            <hr class="hr border-surface-600-400 w-full" />

            <div class="flex w-full flex-col gap-4">
                <div class="flex items-center gap-3">
                    <div
                        class="bg-primary-600-400/20 flex h-10 w-10 items-center justify-center rounded-lg"
                    >
                        <MapPin class="text-primary-600-400" size={20} />
                    </div>
                    <div class="flex flex-col">
                        <span class="text-xs font-bold uppercase opacity-50"> Region </span>
                        <span class="font-bold uppercase">{lobby.region.tag}</span>
                    </div>
                </div>

                <div class="flex items-center gap-3">
                    <div
                        class="bg-secondary-500/20 flex h-10 w-10 items-center justify-center rounded-lg"
                    >
                        <Layers class="text-secondary-500" size={20} />
                    </div>
                    <div class="flex flex-col">
                        <span class="text-xs font-bold uppercase opacity-50"> Rotation </span>
                        <span class="font-bold">{is2A2B ? '2A2B' : 'Default'}</span>
                    </div>
                </div>
            </div>

            {#if isHost}
                <button
                    class="btn preset-filled-error-200-800 mt-4 w-full font-bold"
                    onclick={onLeave}
                >
                    {#if lobbyButtonLoading}
                        <LoaderCircle size={20} class="mr-2 animate-spin" />
                    {:else}
                        <Trash2 size={20} class="mr-2" />
                    {/if}
                    <span>Delete Lobby</span>
                </button>
            {:else if isJoined}
                <button
                    class="btn preset-filled-error-200-800 mt-4 w-full font-bold"
                    onclick={onLeave}
                >
                    {#if lobbyButtonLoading}
                        <LoaderCircle size={20} class="mr-2 animate-spin" />
                    {:else}
                        <LogOut size={20} class="mr-2" />
                    {/if}
                    <span>Leave Lobby</span>
                </button>
            {:else}
                <button
                    class="btn preset-filled-primary-300-700 mt-4 w-full font-bold"
                    onclick={onJoin}
                    disabled={lobby.amountPlayers >= 4}
                >
                    {#if lobbyButtonLoading}
                        <LoaderCircle size={20} class="mr-2 animate-spin" />
                    {:else}
                        <LogIn size={20} class="mr-2" />
                    {/if}
                    <span>Join Lobby</span>
                </button>
            {/if}
        </div>
    </div>
</div>
