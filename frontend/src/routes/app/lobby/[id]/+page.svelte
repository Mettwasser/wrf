<script lang="ts">
    import type { PageProps } from './$types';
    import { useReducer, useTable } from 'spacetimedb/svelte';
    import { reducers, tables } from '$lib/module_bindings';
    import { SenderError } from 'spacetimedb';
    import { goto } from '$app/navigation';
    import { User as UserIcon, Clipboard, ArrowLeft } from 'lucide-svelte';
    import { getRelicImageUrl } from '$lib/utils/relic_image';
    import { getRefinementTextColor } from '$lib/utils/refinement_color';
    import { User } from '$lib/module_bindings/types';
    import { toaster, me as getMe } from '$lib';
    import LobbyInfoCard from '$lib/components/lobby/LobbyInfoCard.svelte';
    import LobbyMemberItem from '$lib/components/lobby/LobbyMemberItem.svelte';
    import LobbyDummyItem from '$lib/components/lobby/LobbyDummyItem.svelte';
    import LobbyOpenSlot from '$lib/components/lobby/LobbyOpenSlot.svelte';
    import type { FullUser } from '$lib/types/full_user';

    let { params }: PageProps = $props();

    const lobbyHostId = Number(params.id);

    const [lobbyTable, lobbyIsReady] = useTable(
        tables.lobby.where((lobby) => lobby.lobbyId.eq(lobbyHostId))
    );
    const lobby = $derived($lobbyTable[0] ?? null);

    const [myBans] = useTable(tables.my_bans);

    let me = getMe();

    const [joinedUsers, joinedUsersReady] = useTable(
        tables.lobby_join
            .where((join) => join.lobbyId.eq(lobbyHostId))
            .rightSemijoin(tables.user, (join, user) => user.id.eq(join.userId))
    );

    const [joinedUserDetails, joinedUserDetailsReady] = useTable(
        tables.lobby_join
            .where((join) => join.lobbyId.eq(lobbyHostId))
            .rightSemijoin(tables.user_details, (join, userDetail) =>
                userDetail.userId.eq(join.userId)
            )
    );

    function moveHostToTop(userList: FullUser[]): FullUser[] {
        const hostIndex = userList.findIndex((u) => u.id === lobbyHostId);

        if (hostIndex > 0) {
            const [host] = userList.splice(hostIndex, 1);
            userList.unshift(host);
        }
        return userList;
    }

    let participants: FullUser[] = $derived(
        moveHostToTop(
            $joinedUsers.map((user) => {
                const details = $joinedUserDetails.find((d) => d.userId === user.id)!;

                return {
                    id: user.id,
                    name: user.name,
                    flags: details.flags.bits,
                    permissions: details.permissions.bits,
                };
            })
        )
    );

    const isHost = $derived(me.current?.user.id === lobbyHostId);
    const isJoined = $derived($joinedUsers.some((user) => user.id === me.current?.user.id));

    const relicUrl = $derived(lobby ? getRelicImageUrl(lobby.activity, lobby.refinement) : '');
    const refinementTextColor = $derived(lobby ? getRefinementTextColor(lobby.refinement) : '');

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
        const isBanned = $myBans.some((row) => row.lobbyId === lobbyHostId);
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
        navigator.clipboard.writeText(lobbyHostId.toString());
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
            description: `You kicked ${user.name} from your Lobby`,
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
            description: `You banned ${user.name} from your Lobby`,
            type: 'info',
        });
        isBanning = false;
    };

    let isAddingDummy = $state(false);
    const addDummyReducer = useReducer(reducers.addDummy);
    const addDummy = async () => {
        isAddingDummy = true;
        await addDummyReducer();
        isAddingDummy = false;
    };

    let isRemovingDummy = $state(false);
    const removeDummyReducer = useReducer(reducers.removeDummy);
    const removeDummy = async () => {
        isRemovingDummy = true;
        await removeDummyReducer();
        isRemovingDummy = false;
    };

    $effect(() => {
        if ($lobbyIsReady && lobby === null) {
            goto('/app');
        }
    });
</script>

<svelte:head>
    {#if participants[0]}
        <title>{participants[0].name}'s Lobby</title>
    {/if}
</svelte:head>

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

    {#if lobby && $joinedUsersReady && $joinedUserDetailsReady}
        <div class="grid w-full grid-cols-1 gap-16 lg:grid-cols-4 lg:gap-0">
            <LobbyInfoCard
                {lobby}
                {relicUrl}
                {refinementTextColor}
                {isHost}
                {isJoined}
                {lobbyButtonLoading}
                onLeave={leaveLobby}
                onJoin={joinLobby}
            />

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
                        <LobbyMemberItem
                            {user}
                            userIsHost={user.id === lobbyHostId}
                            isMe={user.id === me.current?.user.id}
                            {isHost}
                            {isKicking}
                            {isBanning}
                            onKick={kick}
                            onBan={ban}
                        />
                    {/each}

                    {#each Array(lobby.dummies) as _}
                        <LobbyDummyItem {isHost} {isRemovingDummy} onRemove={removeDummy} />
                    {/each}

                    {#each Array(Math.max(0, lobby.lobbySize - participants.length - lobby.dummies)) as _}
                        <LobbyOpenSlot {isHost} {isAddingDummy} onAdd={addDummy} />
                    {/each}
                </div>
            </div>
        </div>
    {/if}
</div>
