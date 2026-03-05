<script lang="ts">
    import { identity } from '$lib';
    import LobbyCreateButton from '$lib/components/LobbyCreateButton.svelte';
    import LobbyItem from '$lib/components/LobbyItem.svelte';
    import { tables } from '$lib/module_bindings/index.js';
    import type { Lobby, User } from '$lib/module_bindings/types.js';
    import type { LobbyAndUser } from '$lib/types/lobby_and_user.js';
    import { useTable } from 'spacetimedb/svelte';
    import { untrack } from 'svelte';

    let { data } = $props();
    let relics = [...data.relics];

    const [lobbies] = useTable(tables.lobby);
    const [users] = useTable(
        tables.user.leftSemijoin(tables.lobby, (user, lobby) => lobby.host.eq(user.id))
    );
    const [joinedLobby, joinedLobbyIsReady] = useTable(
        tables.lobby_join.where((join) => join.user.eq(identity()))
    );

    interface OptionalLobbyAndUser {
        user: User | undefined;
        lobby: Lobby;
    }

    const allLobbiesWithUsers: OptionalLobbyAndUser[] = $derived(
        $lobbies.map((lobby) => ({
            lobby,
            user: $users.find((u) => u.id.equals(lobby.host)),
        }))
    );

    const myLobby: OptionalLobbyAndUser | undefined | null = $derived(
        !$joinedLobbyIsReady
            ? undefined
            : $joinedLobby[0]
              ? allLobbiesWithUsers.find((lu) => lu.lobby.host.equals($joinedLobby[0].host))
              : null
    );

    const lobbiesWithUsers: OptionalLobbyAndUser[] = $derived(
        myLobby === undefined
            ? []
            : myLobby === null
              ? allLobbiesWithUsers
              : allLobbiesWithUsers.filter((lu) => !lu.lobby.host.equals(myLobby.lobby.host))
    );
</script>

<div class="mt-8 flex flex-1 flex-col items-center gap-16">
    <div class="flex w-2/3 flex-col">
        <div class="flex gap-4 max-sm:flex-col">
            <input type="text" class="input" placeholder="Search for a relic" />
            <LobbyCreateButton {relics} hasLobbyOpen={myLobby !== undefined && myLobby !== null} />
        </div>
    </div>
    <ul class="flex w-full flex-col flex-wrap items-center justify-center gap-8 lg:flex-row">
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
    </ul>
</div>
