<script lang="ts">
    import { Clipboard } from 'lucide-svelte';
    import type { SvelteComponent } from 'svelte';
    import { toaster } from '$lib';

    type Props = {
        label?: string;
        value: string;
        icon?: typeof SvelteComponent<any>;
        class?: string;
        inputClass?: string;
    };

    let { label, value, icon: Icon, class: className, inputClass }: Props = $props();

    const copyToClipboard = () => {
        navigator.clipboard.writeText(value);
        toaster.create({
            title: 'Copied!',
            type: 'success',
        });
    };
</script>

<div class="flex flex-col gap-1 {className}">
    {#if label}
        <span class="label-text">{label}</span>
    {/if}
    <div class="flex gap-2">
        <div class="input-group hover:preset-tonal xsm:grid-cols-[auto_1fr] w-full">
            {#if Icon}
                <div class="ig-cell max-xsm:hidden">
                    <Icon />
                </div>
            {/if}
            <input type="text" class="ig-input {inputClass}" {value} disabled />
        </div>

        <div class="flex w-20 gap-2">
            <button
                class="btn-icon preset-filled-surface-900-100 w-full"
                onclick={copyToClipboard}
                title="Copy to clipboard"
            >
                <Clipboard class="size-6" />
            </button>
        </div>
    </div>
</div>
