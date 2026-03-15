<script lang="ts">
    import { User as UserIcon, LoaderCircle, Plus } from 'lucide-svelte';

    interface Props {
        isHost: boolean;
        isAddingDummy: boolean;
        onAdd: () => void;
    }

    let { isHost, isAddingDummy, onAdd }: Props = $props();
</script>

<div
    class="border-surface-600-400/50 card flex justify-between gap-5 border-2 border-dashed p-5 opacity-50 max-md:flex-col md:items-center"
>
    <div class="flex gap-5">
        <div
            class="bg-surface-300-700/50 flex h-14 w-14 items-center justify-center rounded-full max-md:hidden"
        >
            <UserIcon size={28} class="opacity-20" />
        </div>
        <div class="flex flex-col gap-1">
            <span class="text-lg font-bold tracking-wide italic"> Open Slot </span>
            <span class="text-xs tracking-widest uppercase"> Waiting for player... </span>
        </div>
    </div>
    {#if isHost}
        <button
            class="btn md:btn-icon preset-filled-success-300-700 max-md:w-full"
            title="Add Dummy"
            onclick={onAdd}
            disabled={isAddingDummy}
        >
            {#if isAddingDummy}
                <LoaderCircle size={20} class="mr-2 animate-spin" />
            {:else}
                <Plus />
            {/if}
        </button>
    {/if}
</div>
