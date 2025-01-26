<script lang="ts">
    import type { ErrorCollection } from '$lib/utils/error_collection.svelte';
    import type { Snippet } from 'svelte';

    type Props = {
        title: string;
        children: Snippet;
        errorId: number;
        errorCollection: ErrorCollection;
    };

    let { title, children, errorId, errorCollection = $bindable() }: Props = $props();

    let error = $derived(errorCollection.getError(errorId));
</script>

<label class="label">
    <span class="label-text">{title}</span>
    {@render children()}
    {#if error}
        <p class="text-error-300-700 text-sm">{error.errorText}</p>
    {/if}
</label>
