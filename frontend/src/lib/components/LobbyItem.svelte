<script lang="ts">
    import type { LobbyAndUser } from '$lib/types/lobby_and_user';
    import { Tooltip } from '@skeletonlabs/skeleton-svelte';
    import { Check } from 'lucide-svelte';
    import VerifiedBadge from './VerifiedBadge.svelte';
    interface Props {
        lobbyAndUser: LobbyAndUser;
        ownedByMe?: boolean;
    }
    let { lobbyAndUser, ownedByMe = false }: Props = $props();
</script>

<div
    class="card shadow-surface-100/10 flex flex-col gap-6 p-4 shadow-lg lg:w-1/3
    {ownedByMe
        ? 'preset-outlined-success-500 bg-success-200-800/10'
        : 'preset-outlined-surface-600-400 bg-surface-200-800/10'}
    "
>
    {#if ownedByMe}
        <p
            class="bg-success-300-700 text-surface-contrast-300-700 absolute -translate-y-8 rounded-lg p-1 text-sm"
        >
            Your Lobby!
        </p>
    {/if}

    <div class="flex flex-1 justify-between gap-4">
        <div class="flex flex-col gap-2">
            <div class="flex gap-2">
                <h3 class="h3">{lobbyAndUser.lobby.activity}</h3>
                <p class="uppercase">[{lobbyAndUser.lobby.region.tag}]</p>
            </div>
            <p>{lobbyAndUser.lobby.refinement.tag}</p>
        </div>
        <div class="flex flex-col items-center gap-2 text-center">
            {lobbyAndUser.user.username}
            {#if !lobbyAndUser.user.verified}
                <VerifiedBadge />
            {/if}
        </div>
    </div>
    <div class="flex flex-1 justify-between gap-4">
        <span class="text-lg">{lobbyAndUser.lobby.amountPlayers}/{lobbyAndUser.lobby.space}</span>
        {#if !ownedByMe}
            <button type="button" class="btn preset-filled-primary-200-800">Join</button>
        {:else}
            <Tooltip positioning={{ placement: 'top' }} openDelay={200}>
                <Tooltip.Trigger class="underline">
                    <button type="button" class="btn preset-filled-primary-200-800" disabled>
                        Join
                    </button>
                </Tooltip.Trigger>
                <Tooltip.Positioner>
                    <Tooltip.Content class="card preset-filled-surface-300-700 p-4">
                        <p class="dark:text-surface-contrast-700 text-surface-contrast-300">
                            You can't join your own Lobby!
                        </p>
                    </Tooltip.Content>
                </Tooltip.Positioner>
            </Tooltip>
        {/if}
    </div>
</div>
