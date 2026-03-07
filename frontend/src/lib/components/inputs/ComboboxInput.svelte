<script lang="ts">
    import { Check, LoaderCircle, Pen, X } from 'lucide-svelte';
    import type { SvelteComponent } from 'svelte';
    import type { Group } from '$lib/utils/group.svelte';
    import Combobox from '../Combobox.svelte';
    import type { ComboboxData } from '$lib/types/combobox_data';

    type Props = {
        label?: string;
        group: Group<string>;
        icon?: typeof SvelteComponent<any>;
        onSave: () => void;
        onCancel: () => void;
        class?: string;
        data: ComboboxData[];
        placeholder?: string;
        limit?: number;
        displayAsUppercase?: boolean;
    };

    let {
        label,
        group,
        icon: Icon,
        onSave,
        onCancel,
        class: className,
        data,
        placeholder = '',
        limit,
        displayAsUppercase,
    }: Props = $props();

    let comboboxValue = $state([group.value]);

    $effect(() => {
        if (group.isEditing) {
            const val = comboboxValue?.[0] ?? '';
            if (group.value !== val) {
                group.value = val;
            }
        }
    });

    $effect(() => {
        if (!group.isEditing) {
            comboboxValue = [group.value];
        }
    });
</script>

<div class="flex flex-col gap-1 {className}">
    {#if label}
        <span class="label-text">{label}</span>
    {/if}
    <div class="flex gap-2">
        <div
            class="input-group hover:preset-tonal xsm:grid-cols-[auto_1fr] w-full [&_input]:ring-0 [&_input]:focus:outline-none"
        >
            {#if Icon}
                <div class="ig-cell max-xsm:hidden">
                    <Icon />
                </div>
            {/if}
            <Combobox
                {data}
                bind:value={comboboxValue}
                {placeholder}
                {limit}
                {displayAsUppercase}
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
