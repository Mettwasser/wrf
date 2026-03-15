<script lang="ts">
    import { User as UserIcon, LoaderCircle, Minus } from 'lucide-svelte';

    interface Props {
        isHost: boolean;
        isRemovingDummy: boolean;
        onRemove: () => void;
    }

    let { isHost, isRemovingDummy, onRemove }: Props = $props();
</script>

<div
    class="border-surface-600-400/50 card flex justify-between gap-5 border-2 p-5 opacity-75 max-md:flex-col md:items-center"
>
    <div class="flex gap-5">
        <div
            class="bg-surface-300-700/50 flex h-14 w-14 items-center justify-center rounded-full max-md:hidden"
        >
            <UserIcon size={28} class="opacity-75" />
        </div>
        <div class="flex flex-col gap-1">
            <span class="text-lg font-bold tracking-wide italic"> Dummy </span>
            <span class="text-xs tracking-widest uppercase"> Player from an external source </span>
        </div>
    </div>

    {#if isHost}
        <button
            class="btn md:btn-icon preset-filled-error-300-700 max-md:w-full"
            title="Remove Dummy"
            onclick={onRemove}
            disabled={isRemovingDummy}
        >
            {#if isRemovingDummy}
                <LoaderCircle size={20} class="mr-2 animate-spin" />
            {:else}
                <Minus />
            {/if}
        </button>
    {/if}
</div>
