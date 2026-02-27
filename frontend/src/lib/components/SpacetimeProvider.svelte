<script lang="ts">
    import { setContext, type Snippet } from 'svelte';
    import { createSpacetimeDBProvider } from 'spacetimedb/svelte';
    import { DbConnection, type ErrorContext } from '$lib/module_bindings';
    import type { Identity } from 'spacetimedb';

    let { token, children }: { token: string; children: Snippet } = $props();

    const HOST = import.meta.env.VITE_SPACETIMEDB_HOST;
    const DB_NAME = import.meta.env.VITE_SPACETIMEDB_DB_NAME;

    let isConnected = $state(false);

    let identState = $state<{ current: Identity | null }>({ current: null });
    let connState = $state<{ current: DbConnection | null }>({ current: null });

    setContext('ident', identState);
    setContext('conn', connState);

    const onConnect = (conn: DbConnection, identity: Identity, token: string) => {
        console.log('Connected to SpacetimeDB with identity:', identity.toHexString());
        identState.current = identity;
        connState.current = conn;
        isConnected = true;
    };

    const onDisconnect = () => console.log('Disconnected from SpacetimeDB');
    const onConnectError = (ctx: ErrorContext, err: Error) => console.log('Error:', err, ctx);

    // svelte-ignore state_referenced_locally
    const connectionBuilder = DbConnection.builder()
        .withUri(HOST)
        .withDatabaseName(DB_NAME)
        .withToken(token)
        .onConnect(onConnect)
        .onDisconnect(onDisconnect)
        .onConnectError(onConnectError);

    createSpacetimeDBProvider(connectionBuilder);
</script>

{#if isConnected}
    {@render children()}
{:else}
    <div class="flex size-full items-center justify-center">
        <h1 class="h1">Connecting to database...</h1>
    </div>
{/if}
