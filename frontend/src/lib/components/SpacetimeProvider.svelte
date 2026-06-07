<script lang="ts">
    import { setContext, type Snippet } from 'svelte';
    import { createSpacetimeDBProvider } from 'spacetimedb/svelte';
    import { DbConnection, tables, type ErrorContext } from '$lib/module_bindings';
    import { type Identity } from 'spacetimedb';
    import type { Me } from '$lib/module_bindings/types';
    import { goto } from '$app/navigation';

    let { token, children }: { token: string; children: Snippet } = $props();

    const HOST = import.meta.env.VITE_SPACETIMEDB_HOST;
    const DB_NAME = import.meta.env.VITE_SPACETIMEDB_DB_NAME;

    let isConnected = $state(false);

    let identState = $state<{ current: Identity | null }>({ current: null });
    let connState = $state<{ current: DbConnection | null }>({ current: null });
    let meState = $state<{ current: Me | null }>({ current: null });

    setContext('ident', identState);
    setContext('conn', connState);
    setContext('me', meState);

    const onConnect = (conn: DbConnection, identity: Identity, token: string) => {
        console.log('Connected to SpacetimeDB with identity:', identity.toHexString());

        conn.db.me.onUpdate((ctx, oldRow, newRow) => {
            meState.current = newRow;
        });

        conn.db.me.onInsert((ctx, newRow) => {
            if (!meState.current) meState.current = newRow;
        });

        conn.db.me.onDelete((ctx, row) => {
            meState.current = null;
        });

        conn.subscriptionBuilder()
            .onApplied((ctx) => {
                const allMeRows = Array.from(conn.db.me.iter());

                if (allMeRows.length > 0) {
                    meState.current = allMeRows[0];
                } else {
                    meState.current = null;
                }

                isConnected = true;
            })
            .subscribe(tables.me);

        identState.current = identity;
        connState.current = conn;
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

    $effect(() => {
        if (isConnected && meState.current === null) {
            goto('/app/onboarding');
        }
    });
</script>

{#if isConnected}
    {@render children()}
{:else}
    <div class="flex flex-1 items-center justify-center">
        <h1 class="h1 text-center">Connecting to database...</h1>
    </div>
{/if}
