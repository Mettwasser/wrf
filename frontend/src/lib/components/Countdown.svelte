<script lang="ts">
    import type { Snippet } from 'svelte';

    interface Props {
        targetTimestamp: number;
        content: Snippet<[{ formattedTime: string; remainingMs: number }]>;
    }

    let { targetTimestamp, content }: Props = $props();

    let now = $state(Date.now());

    $effect(() => {
        if (now >= targetTimestamp) return;

        const interval = setInterval(() => {
            now = Date.now();
            if (now >= targetTimestamp) {
                clearInterval(interval);
            }
        }, 1000);

        return () => clearInterval(interval);
    });

    let remainingMs = $derived(Math.max(0, targetTimestamp - now));

    let formattedTime = $derived.by(() => {
        const totalSeconds = Math.floor(remainingMs / 1000);
        const minutes = Math.floor(totalSeconds / 60);
        const seconds = totalSeconds % 60;

        return `${minutes}m ${seconds}s`;
    });
</script>

{@render content({ formattedTime, remainingMs })}
