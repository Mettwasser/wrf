<script lang="ts">
    import FormInputGroup from '$lib/components/FormInputGroup.svelte';
    import { fetch, URL } from '$lib';
    import { getContext, onMount } from 'svelte';
    import { isEmailValid } from '$lib/utils/validators';
    import Loader from 'lucide-svelte/icons/loader-circle';
    import { goto } from '$app/navigation';
    import { page } from '$app/state';
    import type { ToastContext } from '@skeletonlabs/skeleton-svelte';

    let password = $state('');
    let passwordRepeat = $state('');
    let errorText: string | null = $state('');
    let isSubmitting = $state(false);

    async function onsubmit(e: Event) {
        e.preventDefault();
        if (password !== passwordRepeat) return (errorText = "The passwords don't match.");
        else errorText = null;

        isSubmitting = true;
        let resp = await fetch(`${URL}/api/auth/reset`, {
            method: 'POST',
            body: JSON.stringify({
                password,
                token: page.params.token,
            }),
            headers: {
                'Content-Type': 'application/json',
            },
        });
        if (resp.ok) goto('/login');
        else errorText = 'Error';
        isSubmitting = false;
    }
</script>

<form class="flex w-3/4 flex-col items-center justify-center gap-4 md:w-1/2 lg:w-[30%]" {onsubmit}>
    <h1 class="h1 text-nowrap">Reset Password</h1>

    <div class="flex w-full flex-col items-center justify-center gap-8">
        <div class="flex w-full flex-col gap-4">
            <FormInputGroup
                title="New Password"
                placeholder="New Password..."
                type="password"
                bind:value={password}
            />
            <FormInputGroup
                title="Repeat New Password"
                placeholder="Repeat New Password..."
                type="password"
                bind:value={passwordRepeat}
            />
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
                    Reset
                {/if}
            </button>
        </div>
    </div>
    {#if errorText}
        <p class="text-error-500">{errorText}</p>
    {/if}
</form>
