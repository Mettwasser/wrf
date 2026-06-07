<script lang="ts">
    import { UserFlags } from '$lib/module_bindings/types';
    import { Bitmask, UserFlags as UserFlagsEnum } from '$lib/utils/bitmask';
    import { BadgeCheck } from 'lucide-svelte';

    let { flags }: { flags: UserFlags } = $props();
</script>

<BadgeCheck
    class="text-success-500 inline size-6 {Bitmask.has(flags.bits, UserFlagsEnum.Verified)
        ? ''
        : 'invisible'}"
/>
