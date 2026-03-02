<script lang="ts">
    import type { User } from '$lib/module_bindings/types';
    import { BadgeCheck } from 'lucide-svelte';

    let { user }: { user: User } = $props();
</script>

<BadgeCheck class="text-success-500! inline size-5 {user.verified ? '' : 'invisible'}" />
