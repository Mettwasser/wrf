<script lang="ts">
    import logo from '$lib/assets/wrf-logo.png';
    import { NavbarDisplayOptions, navbarOpts } from '$lib/index.svelte';
    import { onMount } from 'svelte';

    onMount(() => {
        navbarOpts.display = NavbarDisplayOptions.ThemesOnly;

        return () => (navbarOpts.display = NavbarDisplayOptions.Full);
    });
</script>

<div class="flex flex-1 items-center justify-center gap-[15%]">
    <img
        src={logo}
        alt="logo"
        class="hidden object-scale-down md:flex md:w-[400px] lg:w-[500px] dark:invert"
    />

    <form class="mr-20 flex flex-col items-center justify-center gap-4">
        <h1 class="h1">Register</h1>
        <div class="flex flex-col items-center justify-center gap-4">
            <label class="label">
                <span class="label-text">Warframe Username</span>
                <input class="input" type="text" placeholder="Warframe Username..." />
            </label>
            <label class="label">
                <span class="label-text">Email</span>
                <input class="input" type="email" placeholder="Email..." />
            </label>
            <div class="flex w-full gap-4">
                <label class="label">
                    <span class="label-text">Password</span>
                    <input class="input" type="password" placeholder="Password..." />
                </label>
                <label class="label">
                    <span class="label-text text-nowrap">Repeat Password</span>
                    <input class="input" type="password" placeholder="Repeat Password..." />
                </label>
            </div>

            <div class="mt-8 flex w-full items-center justify-between">
                <p>
                    Already have an account? <br />
                    Login
                    <a href="/login" class="underline">here.</a>
                </p>
                <button class="btn preset-filled-primary-300-700" type="submit">Submit</button>
            </div>
        </div>
        <!-- <p class="mt-16"></p> -->
    </form>
</div>
