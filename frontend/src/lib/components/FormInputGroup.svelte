<script lang="ts">
    import type { HTMLInputAttributes } from 'svelte/elements';

    type Props = { value: string; title: string } & HTMLInputAttributes;

    let { value = $bindable(), title, ...rest }: Props = $props();
</script>

<label class="label">
    <span class="label-text">{title}</span>
    <input class="input" bind:value {...rest} required />
</label>
