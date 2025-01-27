<script lang="ts">
    import type { Lobby, LobbyAndUser } from '$lib/types/lobby';
    import { Tooltip } from '@skeletonlabs/skeleton-svelte';
    interface Props {
        lobbyAndUser: LobbyAndUser;
        ownedByMe?: boolean;
    }
    let { lobbyAndUser, ownedByMe = false }: Props = $props();
</script>

<div
    class="card flex w-2/3 flex-col gap-6 bg-opacity-10 p-4 lg:w-1/3
    {ownedByMe
        ? 'preset-outlined-success-500 bg-success-500'
        : 'preset-outlined-surface-500 bg-surface-500'}
    "
>
    {#if ownedByMe}
        <p
            class="bg-success-300-700 text-success-contrast-300 dark:text-surface-contrast-700 absolute -translate-y-8 rounded-lg p-1 text-sm"
        >
            Your Lobby!
        </p>
    {/if}
    <div class="flex flex-1 justify-between gap-4">
        <div class="flex flex-col gap-2">
            <div class="flex gap-2">
                <h3 class="h3">{lobbyAndUser.lobby.activity}</h3>
                <p>[{lobbyAndUser.lobby.region}]</p>
            </div>
            <p>{lobbyAndUser.lobby.refinement}</p>
        </div>
        <div class="text-center">
            <p class="opacity-25">Host:</p>
            {lobbyAndUser.user.name}
        </div>
    </div>
    <div class="flex flex-1 justify-between gap-4">
        <span class="text-lg">1/{lobbyAndUser.lobby.size}</span>
        {#if !ownedByMe}
            <button type="button" class="btn preset-filled-primary-200-800">Join</button>
        {:else}
            <Tooltip
                positioning={{ placement: 'top' }}
                triggerBase="underline"
                contentBase="card preset-filled-surface-300-700 p-4"
                openDelay={200}
            >
                {#snippet trigger()}
                    <button type="button" class="btn preset-filled-primary-200-800" disabled>
                        Join
                    </button>
                {/snippet}
                {#snippet content()}
                    <p class="dark:text-surface-contrast-700 text-surface-contrast-300">
                        You can't join your own Lobby!
                    </p>
                {/snippet}
            </Tooltip>
        {/if}
    </div>
</div>
