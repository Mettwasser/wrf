<script lang="ts">
    import Info from 'lucide-svelte/icons/info';
    import FormInputGroup from '$lib/components/FormInputGroup.svelte';
    import { fetch, URL } from '$lib';
    import { onMount } from 'svelte';
    import { isEmailValid } from '$lib/utils/validators';
    import Loader from 'lucide-svelte/icons/loader-circle';
    import { dev } from '$app/environment';
    import { Popover } from '@skeletonlabs/skeleton-svelte';
    import { json } from '@sveltejs/kit';
    import DelayedLoader from '$lib/components/DelayedLoader.svelte';

    let { data } = $props();

    let captchaElement: HTMLDivElement | undefined = $state();
    let turnstileResponse: string | undefined = $state(undefined);

    let username = $state('');
    let email = $state('');
    let password = $state('');
    let passwordRepeat = $state('');

    let errorText: string | null = $state('');

    let isSubmitting = $state(false);

    async function onsubmit(e: Event) {
        // stop redirecting; wtf?
        e.preventDefault();
        if (!isEmailValid(email)) return (errorText = 'Please enter a valid email.');
        else if (password !== passwordRepeat) return (errorText = "The passwords don't match.");
        else if (!turnstileResponse && !dev)
            return (errorText = 'Please click the cloudflare box.');
        else errorText = null;

        isSubmitting = true;
        let resp = await fetch(`${URL}/api/auth/register`, {
            method: 'POST',
            body: JSON.stringify({
                name: username,
                email,
                password,
                captchaResponse: turnstileResponse,
            }),
            headers: {
                'Content-Type': 'application/json',
            },
        });
        if (resp.ok) errorText = 'SUCCESS!';
        else errorText = await resp.text();
        isSubmitting = false;
    }

    onMount(() => {
        if (!dev) {
            turnstile.render(captchaElement!, {
                // cSpell:ignore 0x4AAAAAAA4kWe5bbkxGpdPA
                sitekey: '0x4AAAAAAA4kWe5bbkxGpdPA',
                callback: (r) => (turnstileResponse = r),
            });
        }
    });
</script>

{#await data.response}
    <DelayedLoader class="!size-40" />
{:then data}
    <form class=" flex flex-col items-center justify-center gap-4" {onsubmit}>
        <h1 class="h1">Register</h1>
        <p class="flex items-center text-lg">
            Your Code: {data.verificationCode}
            <Popover
                arrow
                contentBase="p-4 !bg-surface-200 dark:!bg-surface-800 sm:w-96 w-60 rounded-md"
                arrowBackground="!bg-surface-200 dark:!bg-surface-800"
            >
                {#snippet trigger()}
                    <button type="button" class="btn-icon">
                        <Info />
                    </button>
                {/snippet}
                {#snippet content()}
                    <p>
                        Rename your currently equipped loadout to this code in Warframe. Note that
                        it might not work first try, as the Warframe Servers update this once every
                        10 minutes.
                    </p>
                {/snippet}
            </Popover>
        </p>
        <div class="flex flex-col items-center justify-center gap-8">
            <div class="flex flex-col gap-4">
                <FormInputGroup
                    title="Warframe Username"
                    placeholder="Warframe Username..."
                    bind:value={username}
                />
                <label class="label">
                    <FormInputGroup title="Email" placeholder="Email..." bind:value={email} />
                </label>
                <div class="flex w-full gap-4">
                    <FormInputGroup
                        type="password"
                        title="Password"
                        placeholder="Password..."
                        bind:value={password}
                    />
                    <FormInputGroup
                        type="password"
                        title="Repeat Password"
                        placeholder="Repeat Password..."
                        bind:value={passwordRepeat}
                    />
                </div>
            </div>

            <div
                bind:this={captchaElement}
                id="hcaptcha"
                data-sitekey="9fbc68b2-1a9a-4140-bd98-c918f1d2211c"
            ></div>

            {#if errorText}
                <p class="text-error-500">{errorText}</p>
            {/if}

            <div class="flex w-full items-center justify-between gap-36">
                <p class="">
                    Already have an account? <br />
                    Login
                    <a href="/login" class="underline">here.</a>
                </p>
                <button
                    class="btn preset-filled-primary-300-700 flex-grow"
                    type="submit"
                    disabled={isSubmitting}
                >
                    {#if isSubmitting}
                        <Loader class="animate-spin" />
                    {:else}
                        Submit
                    {/if}
                </button>
            </div>
        </div>
        <!-- <p class="mt-16"></p> -->
    </form>
{/await}
