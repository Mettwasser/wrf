<script lang="ts">
    import type { ErrorCollection } from '$lib/utils/error_collection.svelte';
    import type { Snippet } from 'svelte';

    type Props = {
        title?: string;
        children: Snippet;
        errorId: number;
        errorCollection: ErrorCollection;
    };

    let { title, children, errorId, errorCollection }: Props = $props();

    let error = $derived(errorCollection.getError(errorId));
</script>

<label class="label">
    {#if title}
        <span class="label-text">{title}</span>
    {/if}
    {@render children()}
    <div class="text-error-300-700 text-xs empty:hidden">{error}</div>
</label>
