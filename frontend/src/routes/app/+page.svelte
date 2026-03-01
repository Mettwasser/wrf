<script lang="ts">
    import { conn as getConn } from '$lib';
    import LobbyCreateButton from '$lib/components/LobbyCreateButton.svelte';
    import LobbyItem from '$lib/components/LobbyItem.svelte';
    import { tables } from '$lib/module_bindings/index.js';
    import type { Lobby, User } from '$lib/module_bindings/types.js';
    import type { LobbyAndUser } from '$lib/types/lobby_and_user.js';
    import { useTable } from 'spacetimedb/svelte';

    let { data } = $props();
    let relics = [...data.relics];

    const [lobbies] = useTable(tables.lobby);
    const [users] = useTable(
        tables.user.leftSemijoin(tables.lobby, (user, lobby) => lobby.host.eq(user.id))
    );

    const lobbiesWithUsers: LobbyAndUser[] = $derived(
        $lobbies.map((lobby) => {
            return {
                lobby,
                user: $users.find((u) => u.id === lobby.host)!,
            };
        })
    );
</script>

<div class="mt-8 flex flex-1 flex-col items-center gap-16">
    <div class="flex w-2/3 flex-col">
        <div class="flex gap-4 max-sm:flex-col">
            <input type="text" class="input" placeholder="Search for a relic" />
            <LobbyCreateButton {relics} />
        </div>
    </div>
    <ul class="flex w-full flex-col flex-wrap items-center justify-center gap-8 lg:flex-row">
        {#each lobbiesWithUsers as lobbyAndUser}
            <LobbyItem {lobbyAndUser} />
        {/each}
    </ul>
</div>
