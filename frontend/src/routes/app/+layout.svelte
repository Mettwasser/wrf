<script lang="ts">
    import { ToastProvider } from '@skeletonlabs/skeleton-svelte';
    import { goto } from '$app/navigation';
    import { useClerkContext } from 'svelte-clerk';
    import SpacetimeProvider from '$lib/components/SpacetimeProvider.svelte';
    import Navbar from '$lib/components/Navbar.svelte';

    const clerkCtx = useClerkContext();
    let token = $state<string | null>(null);

    $effect(() => {
        //  Wait for Clerk to finish initializing. Do nothing until then
        if (!clerkCtx.isLoaded) return;

        // Clerk is loaded. Now check if the user is actually logged in
        if (!clerkCtx.session) {
            goto('/login');
            return;
        }

        // The user is logged in. Fetch the token
        clerkCtx.session.getToken().then((freshToken) => {
            token = freshToken;
        });
    });

    let { children } = $props();
</script>

{#if token}
    <SpacetimeProvider {token}>
        <div class="flex h-auto w-full flex-col">
            <Navbar />

            <ToastProvider placement="bottom-end">
                <div class="flex flex-1 p-4">
                    {@render children()}
                </div>
            </ToastProvider>
        </div>
    </SpacetimeProvider>
{:else}
    <div class="flex size-full items-center justify-center">
        <h1 class="h1">Authenticating...</h1>
    </div>
{/if}
