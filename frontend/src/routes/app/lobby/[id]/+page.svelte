<script lang="ts">
    import type { PageProps } from './$types';
    import { useReducer, useTable } from 'spacetimedb/svelte';
    import { reducers, tables } from '$lib/module_bindings';
    import { and, Identity, SenderError } from 'spacetimedb';
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
        ArrowLeft,
    } from 'lucide-svelte';
    import { getRelicImageUrl } from '$lib/utils/relic_image';
    import { getRefinementTextColor } from '$lib/utils/refinement_color';
    import { RotationType, User } from '$lib/module_bindings/types';
    import VerifiedBadge from '$lib/components/VerifiedBadge.svelte';
    import { identity, toaster } from '$lib';
    import { type ComponentType, type Component } from 'svelte';

    let { params }: PageProps = $props();

    const lobbyHostId = Identity.fromString(params.id);
    const myIdentity = identity();

    const [lobbyTable, lobbyIsReady] = useTable(
        tables.lobby.where((lobby) => lobby.host.eq(lobbyHostId))
    );
    const lobby = $derived($lobbyTable[0] ?? null);

    const [myBans] = useTable(tables.my_bans);

    const [joinedUsers, joinedUsersReady] = useTable(
        tables.lobby_join
            .where((join) => join.host.eq(lobbyHostId))
            .rightSemijoin(tables.user, (join, user) => user.id.eq(join.user))
    );

    function moveHostToTop(userList: User[]): User[] {
        const hostIndex = userList.findIndex((u) => u.id.equals(lobbyHostId));

        if (hostIndex > 0) {
            const [host] = userList.splice(hostIndex, 1);
            userList.unshift(host);
        }
        return userList;
    }

    let participants = $derived(moveHostToTop([...$joinedUsers]));

    const isHost = $derived(myIdentity.equals(lobbyHostId));
    const isJoined = $derived($joinedUsers.some((user) => user.id.equals(myIdentity)));

    const relicUrl = $derived(lobby ? getRelicImageUrl(lobby.activity, lobby.refinement) : '');
    const refinementTextColor = $derived(lobby ? getRefinementTextColor(lobby.refinement) : '');
    const is2A2B = $derived(lobby?.rotationType.tag === RotationType.TwoATwoB.tag);

    const withToasterError = async (fn: () => Promise<void>) => {
        try {
            await fn();
        } catch (e) {
            if (e instanceof SenderError) {
                toaster.create({
                    title: 'Error',
                    description: e.message,
                    type: 'error',
                });
            }
        }
    };

    let isVoluntaryLeavingLobby = $state(false);
    let wasJoined = $state(false);

    let lobbyButtonLoading = $state(false);

    const leaveLobbyReducer = useReducer(reducers.leaveLobby);
    const leaveLobby = async () => {
        // A flag needed to display "you were kicked"
        isVoluntaryLeavingLobby = true;
        lobbyButtonLoading = true;
        await leaveLobbyReducer();
        lobbyButtonLoading = false;
    };

    $effect(() => {
        if (isJoined) {
            wasJoined = true;
            isVoluntaryLeavingLobby = false;
        }
    });

    $effect(() => {
        const isBanned = $myBans.some((row) => row.host.equals(lobbyHostId));
        if (
            wasJoined &&
            joinedUsersReady &&
            !isJoined &&
            !isVoluntaryLeavingLobby &&
            lobby !== null
        ) {
            if (isBanned) {
                toaster.create({
                    title: 'Banned',
                    description: 'You have been banned from the lobby.',
                    type: 'error',
                });
            } else {
                toaster.create({
                    title: 'Kicked',
                    description: 'You have been kicked from the lobby.',
                    type: 'error',
                });
            }
            goto('/app');
        }
    });

    const joinLobbyReducer = useReducer(reducers.joinLobby);
    const joinLobby = async () => {
        lobbyButtonLoading = true;
        withToasterError(() => joinLobbyReducer({ lobbyId: lobbyHostId }));
        lobbyButtonLoading = false;
    };

    const copyLobbyId = () => {
        navigator.clipboard.writeText(lobbyHostId.toHexString());
        toaster.create({
            title: 'Copied!',
            type: 'success',
        });
    };

    let isKicking = $state(false);
    const kickReducer = useReducer(reducers.kick);
    const kick = async (user: User) => {
        isKicking = true;
        await withToasterError(() => kickReducer({ user: user.id }));
        toaster.create({
            title: 'Player Kicked',
            description: `You kicked ${user.username} from your Lobby`,
            type: 'info',
        });
        isKicking = false;
    };

    let isBanning = $state(false);
    const banReducer = useReducer(reducers.ban);
    const ban = async (user: User) => {
        isBanning = true;
        await withToasterError(() => banReducer({ user: user.id }));
        toaster.create({
            title: 'Player Banned',
            description: `You banned ${user.username} from your Lobby`,
            type: 'info',
        });
        isBanning = false;
    };

    $effect(() => {
        if ($lobbyIsReady && lobby === null) {
            goto('/app');
        }
    });
</script>

<svelte:head>
    <title>{participants[0].username}'s Lobby</title>
</svelte:head>

{#snippet actionButton(
    title: string,
    actionFn: (u: User) => void,
    isLoading: boolean,
    IconComponent: Component<any> | ComponentType<any>,
    user: User
)}
    <button
        class="btn-icon preset-filled-error-300-700 max-xsm:w-full"
        {title}
        onclick={() => actionFn(user)}
    >
        {#if isLoading}
            <LoaderCircle size={20} class="mr-2 animate-spin" />
        {:else}
            <IconComponent />
        {/if}
    </button>
{/snippet}

{#snippet statusBadge(label: string, colorPreset: string)}
    <div class="badge {colorPreset} px-3 py-1 text-xs font-black uppercase">
        {label}
    </div>
{/snippet}

<div
    class="container mx-auto flex min-h-full max-w-7xl flex-1 flex-col gap-4 p-4 lg:justify-center lg:p-6"
>
    <div class="bg-surface-100-900/50 preset-outlined-surface-500 card flex p-3">
        <button
            class="btn preset-filled-primary-300-700 max-xsm:w-full"
            onclick={() => window.history.back()}
        >
            <span><ArrowLeft /></span>
            Go Back
        </button>
    </div>
    {#if lobby && joinedUsersReady}
        <div class="grid w-full grid-cols-1 gap-16 lg:grid-cols-4 lg:gap-0">
            <!-- Lobby Info Card -->
            <div class="z-10 flex flex-col lg:col-span-1">
                <div
                    class="card bg-surface-200-800/40 shadow-surface-100/10 preset-outlined-surface-600-400 flex h-full flex-col p-8 shadow-xl backdrop-blur-lg lg:rounded-tr-none lg:rounded-br-none lg:shadow-none"
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
                                class="btn preset-filled-error-200-800 mt-4 w-full font-bold"
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
                                class="btn preset-filled-error-200-800 mt-4 w-full font-bold"
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
                                class="btn preset-filled-primary-300-700 mt-4 w-full font-bold"
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
                class="card bg-surface-100-900/40 preset-outlined-surface-500 flex flex-col gap-8 p-4 backdrop-blur-3xl sm:p-8 lg:col-span-3 lg:rounded-tl-none lg:rounded-bl-none lg:border-l-0"
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
                    {#each participants as user (user.id)}
                        {@const userIsHost = user.id.equals(lobbyHostId)}
                        {@const isMe = user.id.equals(myIdentity)}
                        <div
                            class="card bg-surface-300-700/30 flex flex-col justify-between gap-4 p-5 shadow-md transition-all sm:flex-row sm:items-center
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
                                            <Crown
                                                size={14}
                                                class="text-surface-contrast-600-400"
                                            />
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

                            <div
                                class={[
                                    'flex items-center gap-2',
                                    !(isHost && !userIsHost) && 'max-sm:hidden',
                                ]}
                            >
                                {#if isHost && !userIsHost}
                                    {@render actionButton(
                                        'Kick Player',
                                        kick,
                                        isKicking,
                                        UserMinus,
                                        user
                                    )}
                                    {@render actionButton(
                                        'Ban Player',
                                        ban,
                                        isBanning,
                                        Gavel,
                                        user
                                    )}
                                {:else if userIsHost}
                                    {@render statusBadge('Leader', 'preset-filled-primary-500')}
                                {:else if isMe}
                                    {@render statusBadge('You', 'preset-filled-success-500')}
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
    {/if}
</div>
