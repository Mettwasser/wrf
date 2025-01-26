<script lang="ts">
    import type { Lobby } from '$lib/types/lobby';
    interface Props {
        lobby: Lobby;
    }
    let { lobby }: Props = $props();
</script>

<div
    class="card preset-outlined-surface-500 bg-surface-500 flex w-2/3 flex-col gap-6 bg-opacity-10 p-4 lg:w-1/3"
>
    <div class="flex flex-1 justify-between gap-4">
        <div class="flex flex-col gap-2">
            <div class="flex gap-2">
                <h3 class="h3">{lobby.activity}</h3>
                <p>[{lobby.region}]</p>
            </div>
            <p>{lobby.refinement}</p>
        </div>
        <div class="text-center">
            <p class="opacity-25">Host:</p>
            {lobby.userId}
        </div>
    </div>
    <div class="flex flex-1 justify-between gap-4">
        <span class="text-lg">1/{lobby.size}</span>
        <button type="button" class="btn preset-filled-primary-200-800">Join</button>
    </div>
</div>
