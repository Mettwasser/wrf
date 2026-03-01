<script lang="ts">
    import type { Snippet } from 'svelte';

    type Props = {
        title: string;
        children: Snippet;
    };

    let { title, children }: Props = $props();
</script>

<div class="flex w-1/3 flex-col gap-6">
    <h1 class="h1">{title}</h1>

    {@render children()}
</div>
