<script lang="ts">
    import type { ErrorCollection } from '$lib/utils/error_collection.svelte';
    import type { Snippet } from 'svelte';

    type Props = {
        title: string;
        children: Snippet;
        errorIds: number[];
        errorCollection: ErrorCollection;
    };

    let { title, children, errorIds, errorCollection }: Props = $props();

    let errors = $derived(
        errorIds.map((num) => errorCollection.getError(num)).filter((item) => item !== undefined)
    );
</script>

<label class="label">
    <span class="label-text">{title}</span>
    {@render children()}
    {#each errors as error}
        <p class="text-error-300-700 text-xs">{error}</p>
    {/each}
</label>
