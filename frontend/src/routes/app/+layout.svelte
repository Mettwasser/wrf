<script lang="ts">
    import { Toast } from '@skeletonlabs/skeleton-svelte';
    import { goto } from '$app/navigation';
    import { useClerkContext } from 'svelte-clerk';
    import SpacetimeProvider from '$lib/components/SpacetimeProvider.svelte';
    import Navbar from '$lib/components/Navbar.svelte';
    import { toaster } from '$lib';

    const clerkCtx = useClerkContext();
    let token = $state<string | null>(null);

    $effect(() => {
        //  Wait for Clerk to finish initializing. Do nothing until then
        if (!clerkCtx.isLoaded) return;

        // Clerk is loaded. Now check if the user is actually logged in
        if (!clerkCtx.session) {
            console.log('Session not found, routing to login');
            goto('/login');
            return;
        }

        // The user is logged in. Fetch the token
        clerkCtx.session.getToken().then((freshToken) => {
            token = freshToken;
        });
    });

    let { children } = $props();

    const getToastColors = (type: string | undefined) => {
        switch (type) {
            case 'success':
                return 'preset-filled-success-300-700';
            case 'error':
                return 'preset-filled-error-300-700';
            case 'warning':
                return 'preset-filled-warning-300-700';
            case 'info':
                return 'preset-filled-surface-200-800';
            default:
                return 'preset-filled-primary-300-700';
        }
    };
</script>

{#if token}
    <SpacetimeProvider {token}>
        <div class="flex h-auto w-full flex-col">
            <Navbar />

            <div class="flex min-w-0 flex-1 overflow-y-auto p-4">
                <div class="size-full">
                    {@render children()}
                </div>
            </div>

            <Toast.Group {toaster}>
                {#snippet children(toast)}
                    <Toast {toast} class="text-surface-50 {getToastColors(toast.type)}">
                        <Toast.Message>
                            <Toast.Title>{toast.title}</Toast.Title>
                            <Toast.Description>{toast.description}</Toast.Description>
                        </Toast.Message>
                    </Toast>
                {/snippet}
            </Toast.Group>
        </div>
    </SpacetimeProvider>
{:else}
    <div class="flex size-full items-center justify-center">
        <h1 class="h1">Authenticating...</h1>
    </div>
{/if}
