<script lang="ts">
    import type { PageProps } from './$types';
    import { useReducer, useTable } from 'spacetimedb/svelte';
    import { reducers, tables } from '$lib/module_bindings';
    import { Identity } from 'spacetimedb';
    import { goto } from '$app/navigation';
    import {
        Crown,
        User as UserIcon,
        MapPin,
        Layers,
        LogIn,
        UserMinus,
        Gavel,
        Trash2,
        Clipboard,
        LoaderCircle,
        LogOut,
    } from 'lucide-svelte';
    import { getRelicImageUrl } from '$lib/utils/relic_image';
    import { getRefinementTextColor } from '$lib/utils/refinement_color';
    import { RotationType } from '$lib/module_bindings/types';
    import VerifiedBadge from '$lib/components/VerifiedBadge.svelte';
    import { identity, toaster } from '$lib';

    let { params }: PageProps = $props();

    const lobbyHostId = Identity.fromString(params.id);
    const myIdentity = identity();

    const [lobbyTable, lobbyIsReady] = useTable(
        tables.lobby.where((lobby) => lobby.host.eq(lobbyHostId))
    );
    const lobby = $derived($lobbyTable[0] ?? null);

    const [joinsTable] = useTable(tables.lobby_join.where((join) => join.host.eq(lobbyHostId)));

    const [allUsers] = useTable(tables.user);

    const participants = $derived(
        $allUsers
            .filter((u) => u.id.equals(lobbyHostId) || $joinsTable.some((j) => j.user.equals(u.id)))
            .sort((a, b) => {
                if (a.id.equals(lobbyHostId)) return -1;
                if (b.id.equals(lobbyHostId)) return 1;
                return a.username.localeCompare(b.username);
            })
    );

    const isHost = $derived(myIdentity.equals(lobbyHostId));
    const isJoined = $derived($joinsTable.some((j) => j.user.equals(myIdentity)));

    const relicUrl = $derived(lobby ? getRelicImageUrl(lobby.activity, lobby.refinement) : '');
    const refinementTextColor = $derived(lobby ? getRefinementTextColor(lobby.refinement) : '');
    const is2A2B = $derived(lobby?.rotationType.tag === RotationType.TwoATwoB.tag);

    let lobbyButtonLoading = $state(false);

    const leaveLobbyReducer = useReducer(reducers.leaveLobby);
    const leaveLobby = async () => {
        lobbyButtonLoading = true;
        await leaveLobbyReducer();
        lobbyButtonLoading = false;
    };

    const joinLobbyReducer = useReducer(reducers.joinLobby);
    const joinLobby = async () => {
        lobbyButtonLoading = true;
        await joinLobbyReducer({ lobbyId: lobbyHostId });
        lobbyButtonLoading = false;
    };

    const copyLobbyId = () => {
        navigator.clipboard.writeText(lobbyHostId.toHexString());
        toaster.create({
            title: 'Copied!',
            type: 'success',
        });
    };

    $effect(() => {
        if ($lobbyIsReady && lobby === null) {
            goto('/app');
        }
    });
</script>

{#if lobby}
    <div
        class="container mx-auto flex min-h-full max-w-7xl flex-1 flex-col p-4 lg:justify-center lg:p-8"
    >
        <div class="grid w-full grid-cols-1 gap-16 lg:grid-cols-4 lg:gap-0">
            <!-- Lobby Info Card -->
            <div class="z-10 flex flex-col lg:col-span-1 lg:translate-x-2">
                <div
                    class="card bg-surface-200-800/40 shadow-surface-100/10 preset-outlined-surface-600-400 flex h-full flex-col p-8 shadow-xl backdrop-blur-lg lg:shadow-none"
                >
                    <div class="flex flex-col items-center gap-6">
                        <img
                            src={relicUrl}
                            alt="Relic"
                            class="h-48 w-48 object-contain drop-shadow-2xl"
                        />

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
                                    <span class="text-xs font-bold uppercase opacity-50">
                                        Region
                                    </span>
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
                                    <span class="text-xs font-bold uppercase opacity-50">
                                        Rotation
                                    </span>
                                    <span class="font-bold">{is2A2B ? '2A2B' : 'Default'}</span>
                                </div>
                            </div>
                        </div>

                        {#if isHost}
                            <button
                                class="btn preset-filled-tertiary-200-800 mt-4 w-full font-bold"
                                onclick={leaveLobby}
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
                                class="btn preset-filled-tertiary-200-800 mt-4 w-full font-bold"
                                onclick={leaveLobby}
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
                                class="btn preset-filled-primary-600-400 mt-4 w-full font-bold"
                                onclick={joinLobby}
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

            <!-- Players List -->
            <div
                class="card bg-surface-100-900/40 preset-outlined-surface-500 flex flex-col gap-8 p-4 backdrop-blur-3xl sm:p-8 lg:col-span-3"
            >
                <div class="flex flex-col gap-4">
                    <div class="flex items-center justify-between max-sm:flex-col max-sm:gap-4">
                        <h3 class="h3 flex items-center gap-3 font-bold">
                            <span
                                class="text-surface-50 flex h-10 w-10 items-center justify-center rounded-full"
                            >
                                <UserIcon class="size-8" />
                            </span>
                            Lobby Members
                        </h3>
                        <div>
                            <button
                                class="btn preset-filled-surface-900-100 w-full font-semibold"
                                onclick={copyLobbyId}
                                title="Copy to clipboard"
                            >
                                <Clipboard class="size-6" /> Copy Lobby ID
                            </button>
                        </div>
                    </div>
                    <hr class="hr border-surface-50/50" />
                </div>

                <div class="flex h-full flex-col justify-evenly gap-6">
                    {#each participants as user (user.id.toHexString())}
                        {@const userIsHost = user.id.equals(lobbyHostId)}
                        {@const isMe = user.id.equals(myIdentity)}
                        <div
                            class="card bg-surface-300-700/30 flex flex-col items-center justify-between gap-4 p-5 shadow-md transition-all sm:flex-row
                        {userIsHost
                                ? 'border-primary-600-400/30 border-2'
                                : isMe
                                  ? 'border-success-600-400/30 border-2'
                                  : 'border-surface-600-400/30 border-2'}"
                        >
                            <div class="flex items-center gap-5">
                                <div class="relative max-sm:hidden">
                                    <div
                                        class="bg-surface-300-700/50 flex h-14 w-14 items-center justify-center rounded-full shadow-inner"
                                    >
                                        <UserIcon size={28} class="opacity-80" />
                                    </div>
                                    {#if userIsHost}
                                        <div
                                            class="bg-primary-600-400 absolute -top-1 -right-1 rounded-full p-1.5 shadow-lg"
                                        >
                                            <Crown size={14} class="text-surface-50" />
                                        </div>
                                    {/if}
                                </div>

                                <div class="flex flex-col">
                                    <div class="flex items-center gap-2">
                                        <span
                                            class="text-2xl font-bold wrap-anywhere {userIsHost
                                                ? 'text-primary-300'
                                                : isMe
                                                  ? 'text-success-300/75'
                                                  : 'text-surface-100'}"
                                        >
                                            {user.username}
                                        </span>
                                        <VerifiedBadge {user} />
                                    </div>
                                    <div class="flex items-center gap-2">
                                        {#if userIsHost}
                                            <span
                                                class="text-primary-300/50 text-xs font-bold tracking-widest uppercase"
                                            >
                                                Lobby Host
                                            </span>
                                        {:else if isMe}
                                            <span
                                                class="text-success-300/60 text-xs font-bold tracking-widest uppercase opacity-50"
                                            >
                                                You
                                            </span>
                                        {:else}
                                            <span
                                                class="text-xs font-bold tracking-widest uppercase opacity-50"
                                            >
                                                Member
                                            </span>
                                        {/if}
                                    </div>
                                </div>
                            </div>

                            <div class="flex items-center gap-2">
                                {#if isHost && !userIsHost}
                                    <button
                                        class="btn-icon preset-filled-error-300-700"
                                        title="Kick Player"
                                    >
                                        <UserMinus />
                                    </button>
                                    <button
                                        class="btn-icon preset-filled-error-300-700"
                                        title="Ban Player"
                                    >
                                        <Gavel />
                                    </button>
                                {:else if userIsHost}
                                    <div
                                        class="badge preset-filled-primary-500 hidden px-3 py-1 text-xs font-black uppercase sm:block"
                                    >
                                        Leader
                                    </div>
                                {:else if isMe}
                                    <div
                                        class="badge preset-filled-success-500 hidden px-3 py-1 text-xs font-black uppercase sm:block"
                                    >
                                        You
                                    </div>
                                {/if}
                            </div>
                        </div>
                    {/each}

                    {#each Array(Math.max(0, lobby.lobbySize - participants.length)) as _}
                        <div
                            class="border-surface-600-400/50 card flex items-center gap-5 border-2 border-dashed p-5 opacity-50"
                        >
                            <div
                                class="bg-surface-300-700/50 flex h-14 w-14 items-center justify-center rounded-full"
                            >
                                <UserIcon size={28} class="opacity-20" />
                            </div>
                            <div class="flex flex-col gap-1">
                                <span class="text-lg font-bold tracking-wide italic">
                                    Open Slot
                                </span>
                                <span class="text-xs tracking-widest uppercase">
                                    Waiting for player...
                                </span>
                            </div>
                        </div>
                    {/each}
                </div>
            </div>
        </div>
    </div>
{/if}
