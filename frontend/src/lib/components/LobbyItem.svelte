<script lang="ts">
    import type { LobbyAndUser } from '$lib/types/lobby_and_user';
    import { User as UserIcon } from 'lucide-svelte';
    import VerifiedBadge from './VerifiedBadge.svelte';
    import { getRelicImageUrl } from '$lib/utils/relic_image';
    import { getRefinementTextColor } from '$lib/utils/refinement_color';
    import { RotationType } from '$lib/module_bindings/types';

    interface Props {
        lobbyAndUser: LobbyAndUser;
        ownedByMe?: boolean;
    }

    let { lobbyAndUser, ownedByMe = false }: Props = $props();

    const relicUrl = $derived(
        getRelicImageUrl(lobbyAndUser.lobby.activity, lobbyAndUser.lobby.refinement)
    );

    const refinementTextColor = $derived(getRefinementTextColor(lobbyAndUser.lobby.refinement));

    const is2A2B = $derived(lobbyAndUser.lobby.rotationType.tag === RotationType.TwoATwoB.tag);
</script>

<div
    class="card bg-surface-200-800/40 shadow-surface-100/10 preset-outlined-surface-600-400 hover:bg-success-300-700/15 relative grid w-1/4 cursor-pointer grid-cols-3 gap-4 border-2! p-4 shadow-lg backdrop-blur-lg"
>
    {#if ownedByMe}
        <p
            class="bg-success-300-700 text-surface-contrast-700 absolute -top-4 left-4 rounded-lg px-2 py-1 text-xs font-bold shadow-sm"
        >
            Your Lobby!
        </p>
    {/if}

    <div class="col-span-1 flex flex-col justify-between gap-4 text-[okl]">
        <div class="flex flex-col gap-1">
            <h3 class="h3 leading-tight">{lobbyAndUser.lobby.activity}</h3>
            <span class="{refinementTextColor} text-lg">
                <strong>{lobbyAndUser.lobby.refinement.tag}</strong>
            </span>
        </div>

        <hr class="hr border-surface-600-400" />

        <div class="flex flex-col items-start gap-2">
            <div class="flex items-center gap-2 text-lg">
                <strong>{lobbyAndUser.user.username}</strong>
                <VerifiedBadge user={lobbyAndUser.user} />
            </div>
            <div class="flex gap-2">
                <div class="card preset-outlined-surface-300-700 flex w-fit items-center p-1">
                    {#each Array(4) as _, i (i)}
                        {@const idx = i + 1}
                        <UserIcon
                            size={18}
                            class="{lobbyAndUser.lobby.space < idx ? 'invisible' : ''} 
                                {lobbyAndUser.lobby.amountPlayers >= idx
                                ? 'text-success-500'
                                : 'opacity-50'}"
                        />
                    {/each}
                </div>
                •
                <span
                    class="badge preset-filled-primary-300-700 text-xs font-semibold uppercase opacity-70"
                >
                    {lobbyAndUser.lobby.region.tag}
                </span>
                {#if is2A2B}
                    <span class="badge preset-filled-secondary-300-700 font-semibold opacity-70">
                        2A2B
                    </span>
                {/if}
            </div>
        </div>
    </div>

    <div class="col-span-2 flex items-center justify-center p-2">
        <img
            src={relicUrl}
            alt="Relic"
            class="h-40 w-full object-contain object-right drop-shadow-md"
        />
    </div>
</div>
