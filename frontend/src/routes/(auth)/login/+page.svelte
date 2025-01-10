<script lang="ts">
    import FormInputGroup from '$lib/components/FormInputGroup.svelte';
    import { URL } from '$lib';
    import { onMount } from 'svelte';
    import { isEmailValid } from '$lib/utils/validators';
    import Loader from 'lucide-svelte/icons/loader-circle';
    import { goto } from '$app/navigation';

    let email = $state('');
    let password = $state('');

    let errorText: string | null = $state('');

    let isSubmitting = $state(false);

    async function onsubmit(e: Event) {
        // stop redirecting; wtf?
        e.preventDefault();
        if (!isEmailValid(email)) return (errorText = 'Please enter a valid email.');
        else errorText = null;

        isSubmitting = true;
        let resp = await fetch(`${URL}/api/auth/login`, {
            method: 'POST',
            body: JSON.stringify({
                email,
                password,
            }),
            headers: {
                'Content-Type': 'application/json',
            },
            credentials: 'include',
        });
        if (resp.ok) goto('/app');
        else if (resp.status == 400 || resp.status == 401)
            errorText =
                'Either the email is not registered, the password is wrong, or the email has not been verified.';
        isSubmitting = false;
    }
</script>

<form class="flex w-3/4 flex-col items-center justify-center gap-4 md:w-1/2 lg:w-[30%]" {onsubmit}>
    <h1 class="h1">Login</h1>

    <div class="flex w-full flex-col items-center justify-center gap-8">
        <div class="flex w-full flex-col gap-4">
            <FormInputGroup title="Email" placeholder="Email..." type="email" bind:value={email} />
            <div class="flex w-full gap-4">
                <label class="label">
                    <span class="flex justify-between">
                        <p class="label-text">Password</p>
                        <a class="label-text text-secondary-700-300" href="/forgot">
                            Forgot Password?
                        </a>
                    </span>
                    <input
                        class="input"
                        type="password"
                        title="Password"
                        placeholder="Password..."
                        bind:value={password}
                    />
                </label>
            </div>
        </div>

        <div class="flex w-full items-center justify-between gap-36">
            <button
                class="btn preset-filled-primary-300-700 w-full"
                type="submit"
                disabled={isSubmitting}
            >
                {#if isSubmitting}
                    <Loader class="animate-spin" />
                {:else}
                    Login
                {/if}
            </button>
        </div>
    </div>
    {#if errorText}
        <p class="text-error-500">{errorText}</p>
    {/if}
</form>
