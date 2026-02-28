<script lang="ts">
    import { Portal, Tooltip } from '@skeletonlabs/skeleton-svelte';
    import type { Snippet } from 'svelte';
    import type { Placement } from '@floating-ui/dom';

    interface Props {
        children: Snippet;
        text: Snippet | string;
        placement?: Placement | undefined;
    }

    let { children, text, placement }: Props = $props();
</script>

<Tooltip positioning={{ placement: placement }}>
    <Tooltip.Trigger>
        {@render children()}
    </Tooltip.Trigger>
    <Portal>
        <Tooltip.Positioner>
            <Tooltip.Content class="card preset-filled-surface-300-700 p-4 text-center">
                {#if typeof text === 'string'}
                    {@html text}
                {:else}
                    {@render text()}
                {/if}
                <Tooltip.Arrow
                    class="[--arrow-background:var(--color-surface-300-700)] [--arrow-size:--spacing(2)]"
                >
                    <Tooltip.ArrowTip />
                </Tooltip.Arrow>
            </Tooltip.Content>
        </Tooltip.Positioner>
    </Portal>
</Tooltip>
