<script lang="ts">
    import type { Snippet } from 'svelte';

    type Props = {
        title: string;
        children: Snippet;
        class?: string;
    };

    let { title, children, class: className = '' }: Props = $props();
</script>

<div class="flex w-full flex-col gap-6 md:w-2/3 xl:w-1/3 {className}">
    <h1 class="h1 xsm:text-nowrap">{title}</h1>

    {@render children()}
</div>
