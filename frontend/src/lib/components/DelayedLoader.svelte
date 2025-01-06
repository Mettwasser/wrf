<script lang="ts">
    import type { IconProps } from 'lucide-svelte';
    import Loader from 'lucide-svelte/icons/loader-circle';

    let { delay = 500, ...props }: { delay?: number } & IconProps = $props();

    let show = $state(false);

    setTimeout(() => (show = true), delay);
</script>

{#if show}
    <Loader {...props} class="animate-spin {props.class}" />
{/if}
