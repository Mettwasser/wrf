<script lang="ts">
    import Combobox from '$lib/components/Combobox.svelte';
    import LobbyCreateButton from '$lib/components/LobbyCreateButton.svelte';
    import LobbyItem from '$lib/components/LobbyItem.svelte';
    import { mock as originalMock } from '$lib/types/lobby';
    import { ClientEvent, io, ServerEvent, type SubscriptionType } from '$lib/utils/socket';
    import { onMount } from 'svelte';

    let { data } = $props();

    // TODO: replace mock with actual data
    const mock = $state(originalMock);

    let relics = [...data.relics];

    onMount(() => {
        const subscribeData: SubscriptionType = {
            type: 'Recent',
        };
        io.emit(ClientEvent.Subscribe, subscribeData);

        io.on(ServerEvent.CreateLobby, (lobby) => {
            mock.unshift(lobby);
        });

        return () => {
            io.emit(ClientEvent.Unsubscribe, subscribeData);
        };
    });
</script>

<div class="mt-8 flex flex-1 flex-col items-center gap-16">
    <div class="flex w-2/3 flex-col">
        <div class="flex gap-4 max-sm:flex-col">
            <input type="text" class="input" placeholder="Search for a relic" />
            <LobbyCreateButton {relics} />
        </div>
    </div>
    <ul class="flex w-full flex-col flex-wrap items-center justify-center gap-8 lg:flex-row">
        {#each mock as lobby}
            <LobbyItem {lobby} />
        {/each}
    </ul>
</div>
