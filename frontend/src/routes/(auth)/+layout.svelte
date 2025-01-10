<script lang="ts">
    import { ToastProvider } from '@skeletonlabs/skeleton-svelte';
    import logo from '$lib/assets/wrf-logo.png';
    import ThemeButtons from '$lib/components/ThemeButtons.svelte';

    let { children } = $props();
</script>

<div class="flex size-full flex-col">
    <div class="absolute flex w-full justify-end gap-4 p-4">
        <ThemeButtons />
    </div>

    <ToastProvider placement="bottom-end">
        <div class="flex flex-1 items-center justify-center gap-[10%]">
            <img
                src={logo}
                alt="logo"
                class="hidden object-scale-down lg:flex lg:w-[500px] dark:invert"
            />
            {@render children()}
        </div>
    </ToastProvider>
</div>
