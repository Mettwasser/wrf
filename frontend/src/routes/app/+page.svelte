<script lang="ts">
    import { me as getMe } from '$lib';
    import LobbyCreateButton from '$lib/components/LobbyCreateButton.svelte';
    import LobbyItem from '$lib/components/LobbyItem.svelte';
    import { tables } from '$lib/module_bindings/index.js';
    import type { Lobby } from '$lib/module_bindings/types.js';
    import { Region, RelicRefinement } from '$lib/module_bindings/types.js';
    import type { LobbyAndUser } from '$lib/types/lobby_and_user.js';
    import { useTable } from 'spacetimedb/svelte';
    import { Dialog, Portal } from '@skeletonlabs/skeleton-svelte';
    import FilterModal from '$lib/components/modals/FilterModal.svelte';
    import { Funnel, LoaderCircle, Search } from 'lucide-svelte';
    import { Debounced } from 'runed';
    import type { FullUser } from '$lib/types/full_user.js';
    import SpacetimeProvider from '$lib/components/SpacetimeProvider.svelte';

    let { data } = $props();

    let relics = $derived(data.relics);

    let me = getMe();
    const myId = me.current?.user.id || 0;

    const [lobbies, lobbiesAreReady] = useTable(tables.lobby);
    const [users, usersAreReady] = useTable(
        tables.user.leftSemijoin(tables.lobby, (user, lobby) => lobby.lobbyId.eq(user.id))
    );
    const [joinedLobby, joinedLobbyIsReady] = useTable(
        tables.lobby_join.where((join) => join.userId.eq(myId))
    );
    const [userDetails, userDetailsAreReady] = useTable(
        tables.user_details.leftSemijoin(tables.lobby, (details, lobby) =>
            details.userId.eq(lobby.lobbyId)
        )
    );

    const [myBans, myBansAreReady] = useTable(tables.my_bans);

    interface OptionalLobbyAndUser {
        user: FullUser | undefined;
        lobby: Lobby;
    }

    let searchInput = $state('');
    let relicFilter = new Debounced(() => searchInput, 300);

    let eraFilter = $state<string[]>([]);
    let refinementFilter = $state<RelicRefinement['tag'][]>([]);
    let regionFilter = $state<Region['tag'][]>([]);
    let only2A2B: boolean | undefined = $state(undefined);
    let showFilterModal = $state(false);

    const allLobbiesWithUsers: OptionalLobbyAndUser[] = $derived(
        $lobbies
            .filter((lobby) => {
                const matchesRelic =
                    relicFilter.current === '' ||
                    lobby.activity.toLowerCase().includes(relicFilter.current.toLowerCase());
                const matchesEra =
                    eraFilter.length === 0 ||
                    eraFilter.some((era) =>
                        lobby.activity.toLowerCase().startsWith(era.toLowerCase())
                    );
                const matchesRefinement =
                    refinementFilter.length === 0 ||
                    refinementFilter.includes(lobby.refinement.tag);
                const matchesRegion =
                    regionFilter.length === 0 || regionFilter.includes(lobby.region.tag);

                const matchesRotation =
                    only2A2B === undefined ||
                    (only2A2B && lobby.rotationType.tag === 'TwoATwoB') ||
                    (!only2A2B && lobby.rotationType.tag === 'FourA');

                return (
                    matchesRelic &&
                    matchesEra &&
                    matchesRefinement &&
                    matchesRegion &&
                    matchesRotation &&
                    !$myBans.find((ban) => ban.lobbyId === lobby.lobbyId)
                );
            })
            .map((lobby) => {
                let user = $users.find((u) => u.id === lobby.lobbyId)!;
                let details = $userDetails.find((d) => d.userId === lobby.lobbyId)!;
                return {
                    lobby,
                    user: {
                        id: user.id,
                        flags: details.flags.bits,
                        name: user.name,
                        permissions: details.permissions.bits,
                    },
                };
            })
    );

    const myLobby: OptionalLobbyAndUser | undefined | null = $derived(
        !($joinedLobbyIsReady && $userDetailsAreReady)
            ? undefined
            : $joinedLobby[0]
              ? allLobbiesWithUsers.find((lu) => lu.lobby.lobbyId === $joinedLobby[0].lobbyId)
              : null
    );

    const lobbiesWithUsers: OptionalLobbyAndUser[] = $derived(
        myLobby === undefined
            ? []
            : myLobby === null
              ? allLobbiesWithUsers
              : allLobbiesWithUsers.filter((lu) => lu.lobby.lobbyId !== myLobby.lobby.lobbyId)
    );

    let activeFilterCount = $derived(
        (eraFilter.length > 0 ? 1 : 0) +
            (refinementFilter.length > 0 ? 1 : 0) +
            (regionFilter.length > 0 ? 1 : 0) +
            (only2A2B !== undefined ? 1 : 0)
    );
</script>

<svelte:head>
    <title>Lobby Browser</title>
</svelte:head>

<div class="mt-8 flex flex-1 flex-col items-center gap-16">
    <div class="xsm:w-4/5 flex w-full flex-col px-2 sm:w-3/5 lg:w-2/5">
        <div class="lg flex w-full flex-col gap-4 xl:flex-row">
            <div class="input-group w-full sm:grid-cols-[auto_1fr_auto]">
                <div
                    class="ig-cell preset-outlined-surface-400-600 bg-surface-300-700/20 hidden border-r-0 sm:flex"
                >
                    <Search />
                </div>
                <input
                    type="text"
                    class="ig-input preset-outlined-surface-400-600 bg-surface-300-700/20 w-full"
                    placeholder="Search for a relic"
                    bind:value={searchInput}
                />
            </div>

            <Dialog open={showFilterModal} onOpenChange={(e) => (showFilterModal = e.open)}>
                <Dialog.Trigger
                    class="btn preset-outlined-surface-400-600 bg-surface-300-700/20 relative h-full"
                >
                    <Funnel class="mr-2 size-4" />
                    Filters
                    {#if activeFilterCount > 0}
                        <span
                            class="badge-icon preset-filled-primary-300-700 absolute -top-3 -right-3 size-4"
                        >
                            {activeFilterCount}
                        </span>
                    {/if}
                </Dialog.Trigger>
                <Portal>
                    <Dialog.Backdrop class="bg-surface-50-950/50 fixed inset-0 z-50" />
                    <Dialog.Positioner
                        class="fixed inset-0 z-50 flex items-center justify-center px-2"
                    >
                        <Dialog.Content
                            class="card bg-surface-100-900 w-lg space-y-4 p-8 shadow-xl "
                        >
                            <FilterModal
                                bind:open={showFilterModal}
                                bind:eraFilter
                                bind:refinementFilter
                                bind:regionFilter
                                bind:only2A2B
                            />
                        </Dialog.Content>
                    </Dialog.Positioner>
                </Portal>
            </Dialog>

            <LobbyCreateButton {relics} hasLobbyOpen={myLobby !== undefined && myLobby !== null} />
        </div>
    </div>
    <ul class="flex w-full flex-col flex-wrap items-center justify-center gap-8 lg:flex-row">
        {#if $myBansAreReady && $lobbiesAreReady && $usersAreReady && $joinedLobbyIsReady}
            {#if myLobby && myLobby.user}
                {@const myLobbyAndUser = myLobby as LobbyAndUser}
                <LobbyItem lobbyAndUser={myLobbyAndUser} ownedByMe />
            {/if}
            {#each lobbiesWithUsers as item}
                {#if item.user}
                    {@const lobbyAndUser = item as LobbyAndUser}
                    <LobbyItem {lobbyAndUser} />
                {/if}
            {/each}
        {:else}
            <LoaderCircle class="size-6 animate-spin" />
        {/if}
    </ul>
</div>
