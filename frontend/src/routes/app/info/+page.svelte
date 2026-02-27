<script lang="ts">
    import { conn, identity } from '$lib';
    import { CheckCircle, User, X } from 'lucide-svelte';
    import Section from './Section.svelte';

    let me = conn().db.user.id.find(identity());
</script>

<div class="flex flex-1 flex-col items-center justify-center gap-32">
    <Section title="Username" value={me?.username} Icon={User} />

    <Section title="Verification">
        {#if me?.verified}
            <X class="size-10 text-error-500" />
        {:else}
            <CheckCircle class="size-10 text-success-500" />
        {/if}
    </Section>
</div>
