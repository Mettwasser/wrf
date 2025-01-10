<script lang="ts">
    import { Mail, Pen } from 'lucide-svelte';

    let { data } = $props();
</script>

<div class="flex flex-1 items-center justify-center gap-32">
    <div class="flex w-1/3 flex-col">
        <h1 class="h1">Username</h1>
        <div class="input-group hover:preset-tonal xsm:flex hidden w-full grid-cols-[auto_1fr]">
            <div class="input-group-cell">
                <Pen size={18} />
            </div>
            <input type="text" class="input text-center" value={data.user.name} />
        </div>
    </div>
    <div class="flex w-1/3 flex-col">
        <h1 class="h1">Email</h1>
        <div class="input-group hover:preset-tonal xsm:flex hidden w-full grid-cols-[auto_1fr]">
            <div class="input-group-cell">
                <Mail size={18} />
            </div>
            <input type="text" class="input text-center" value={data.user.email} />
        </div>
    </div>
</div>
