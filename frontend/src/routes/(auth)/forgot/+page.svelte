<script lang="ts">
    import FormInputGroup from '$lib/components/FormInputGroup.svelte';
    import { fetch, URL } from '$lib';
    import { getContext } from 'svelte';
    import { isEmailValid } from '$lib/utils/validators';
    import Loader from 'lucide-svelte/icons/loader-circle';
    import type { ToastContext } from '@skeletonlabs/skeleton-svelte';

    let email = $state('');
    let errorText: string | null = $state('');
    let isSubmitting = $state(false);

    const toast: ToastContext = getContext('toast');

    async function onsubmit(e: Event) {
        // stop redirecting; wtf?
        e.preventDefault();
        if (!isEmailValid(email)) return (errorText = 'Please enter a valid email.');
        else errorText = null;

        isSubmitting = true;
        let resp = await fetch(`${URL}/api/auth/forgot`, {
            method: 'POST',
            body: JSON.stringify({
                email,
            }),
            headers: {
                'Content-Type': 'application/json',
            },
        });
        if (resp.ok)
            toast.create({
                title: 'Email sent!',
                description: 'The email has been successfully sent. Check your emails.',
                type: 'success',
                duration: 5000,
            });
        else if (resp.status == 400 || resp.status == 401)
            errorText =
                'Either the email is not registered, the password is wrong, or the email has not been verified.';
        isSubmitting = false;
    }
</script>

<form class="flex w-3/4 flex-col items-center justify-center gap-4 md:w-1/2 lg:w-[30%]" {onsubmit}>
    <h1 class="h1 text-nowrap">Forgot Password</h1>

    <div class="flex w-full flex-col items-center justify-center gap-8">
        <div class="flex w-full flex-col gap-4">
            <FormInputGroup title="Email" placeholder="Email..." type="email" bind:value={email} />
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
                    Send E-Mail
                {/if}
            </button>
        </div>
    </div>
    {#if errorText}
        <p class="text-error-500">{errorText}</p>
    {/if}
</form>
