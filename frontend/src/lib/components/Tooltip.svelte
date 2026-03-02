<script lang="ts">
    import { Portal, Tooltip, type TooltipRootProps } from '@skeletonlabs/skeleton-svelte';
    import type { Snippet } from 'svelte';
    import type { Placement } from '@floating-ui/dom';

    interface Props {
        children: Snippet;
        text: Snippet | string;
        placement?: Placement | undefined;
        options?: Omit<TooltipRootProps, 'positioning'>;
    }

    let { children, text, placement, options }: Props = $props();
</script>

<Tooltip positioning={{ placement: placement }} {...options}>
    <Tooltip.Trigger onclick={(e) => e.preventDefault()}>
        {@render children()}
    </Tooltip.Trigger>
    <Portal>
        <Tooltip.Positioner class="z-50!">
            <Tooltip.Content class="card preset-filled-surface-200-800 p-4 text-center shadow-xl">
                {#if typeof text === 'string'}
                    {@html text}
                {:else}
                    {@render text()}
                {/if}
                <Tooltip.Arrow
                    class="[--arrow-background:var(--color-surface-200-800)] [--arrow-size:--spacing(2)]"
                >
                    <Tooltip.ArrowTip />
                </Tooltip.Arrow>
            </Tooltip.Content>
        </Tooltip.Positioner>
    </Portal>
</Tooltip>
