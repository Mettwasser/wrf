<script lang="ts">
    import type { LobbyAndUser } from '$lib/types/lobby_and_user';
    import { Check } from 'lucide-svelte';
    import VerifiedBadge from './VerifiedBadge.svelte';
    import Tooltip from './Tooltip.svelte';
    import { getRelicImageUrl } from '$lib/utils/relic_image';
    interface Props {
        lobbyAndUser: LobbyAndUser;
        ownedByMe?: boolean;
    }
    let { lobbyAndUser, ownedByMe = false }: Props = $props();

    const relicUrl = $derived(
        getRelicImageUrl(lobbyAndUser.lobby.activity, lobbyAndUser.lobby.refinement)
    );
</script>

<div
    class="
    card shadow-surface-100/10 preset-outlined-surface-600-400 bg-surface-200-800/40 flex w-1/4 flex-col gap-6 border-2! p-4 shadow-lg backdrop-blur-lg
    "
>
    {#if ownedByMe}
        <p
            class="bg-success-300-700 text-success-contrast-300 dark:text-surface-contrast-700 absolute -translate-y-8 rounded-lg p-1 text-xs"
        >
            Your Lobby!
        </p>
    {/if}

    <div class="flex flex-1 justify-between gap-4">
        <div class="flex flex-col gap-4">
            <div>
                <div class="flex gap-2">
                    <h3 class="h3">{lobbyAndUser.lobby.activity}</h3>
                </div>
                <span class="text-primary-500">{lobbyAndUser.lobby.refinement.tag}</span>
            </div>
            <div class="text-tertiary-300 flex items-center gap-2">
                {lobbyAndUser.user.username}
                <VerifiedBadge user={lobbyAndUser.user} />
            </div>

            <div>
                <span class="uppercase">
                    {lobbyAndUser.lobby.region.tag}
                </span>
            </div>
        </div>
        <div class="flex items-start text-center text-lg">
            <img src={relicUrl} alt="Relic" class="h-30 object-contain" />
        </div>
    </div>
    <div class="flex flex-1 justify-between gap-4">
        <span class="text-lg">{lobbyAndUser.lobby.amountPlayers}/{lobbyAndUser.lobby.space}</span>
        {#if !ownedByMe}
            <button type="button" class="btn preset-filled-primary-200-800">Join</button>
        {:else}
            <Tooltip>
                <button type="button" class="btn preset-filled-primary-200-800" disabled>
                    Join
                </button>

                {#snippet text()}
                    <span>You can't join your own lobby!</span>
                {/snippet}
            </Tooltip>
        {/if}
    </div>
</div>
