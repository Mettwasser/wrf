<script lang="ts">
    import logo from '$lib/assets/wrf-logo.png';
    let { children } = $props();
</script>

<div class="flex flex-1 items-center justify-center gap-[10%]">
    <img src={logo} alt="logo" class="hidden object-scale-down lg:flex lg:w-[500px] dark:invert" />
    {@render children()}
</div>
