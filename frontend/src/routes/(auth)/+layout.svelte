<script lang="ts">
    import logo from '$lib/assets/wrf-logo.png';
    import ThemeButtons from '$lib/components/ThemeButtons.svelte';

    let { children } = $props();
</script>

<div class="flex flex-1 flex-col">
    <div class="absolute flex w-full justify-end gap-4 p-4">
        <ThemeButtons />
    </div>

    <div class="flex flex-1 items-center justify-center gap-[10%]">
        <img src={logo} alt="logo" class="hidden object-scale-down lg:flex lg:w-125 dark:invert" />
        {@render children()}
    </div>
</div>
