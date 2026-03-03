<script lang="ts">
    import { Check, LoaderCircle, Pen, X } from 'lucide-svelte';
    import type { SvelteComponent } from 'svelte';
    import type { Group } from '$lib/utils/group.svelte';

    type Props = {
        label?: string;
        group: Group<string>;
        icon?: typeof SvelteComponent<any>;
        onSave: () => void;
        onCancel: () => void;
        class?: string;
    };

    let { label, group, icon: Icon, onSave, onCancel, class: className }: Props = $props();
</script>

<div class="flex flex-col gap-1 {className}">
    {#if label}
        <span class="label-text">{label}</span>
    {/if}
    <div class="flex gap-2">
        <div class="input-group hover:preset-tonal w-full grid-cols-[auto_1fr]">
            {#if Icon}
                <div class="ig-cell">
                    <Icon />
                </div>
            {/if}
            <input
                type="text"
                class="ig-input"
                bind:value={group.value}
                disabled={!group.isEditing}
            />
        </div>
        <div class="flex w-28 gap-2">
            {#if !group.isEditing}
                <button
                    class="btn-icon preset-filled-surface-900-100 w-full"
                    onclick={group.toggleEditing}
                >
                    <Pen class="size-6" />
                </button>
            {:else}
                <button class="btn-icon preset-filled-success-300-700 w-full" onclick={onSave}>
                    {#if group.isSaving}
                        <LoaderCircle class="size-6 animate-spin" />
                    {:else}
                        <Check class="size-6" />
                    {/if}
                </button>
                <button class="btn-icon preset-filled-error-300-700 w-full" onclick={onCancel}>
                    <X class="size-6" />
                </button>
            {/if}
        </div>
    </div>
    {#if group.errorText}
        <p class="text-error-300-700 text-xs">{group.errorText}</p>
    {/if}
</div>
