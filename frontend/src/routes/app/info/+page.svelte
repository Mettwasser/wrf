<script lang="ts">
    import {
        BadgeCheck,
        Check,
        ChevronDownIcon,
        Clipboard,
        Hash,
        IdCard,
        LoaderCircle,
        Pen,
        User,
        X,
    } from 'lucide-svelte';
    import Section from './Section.svelte';
    import { useReducer, useTable } from 'spacetimedb/svelte';
    import { reducers, tables } from '$lib/module_bindings';
    import { group } from '$lib/utils/group.svelte';
    import type { UserVerification } from '$lib/module_bindings/types';
    import { Accordion } from '@skeletonlabs/skeleton-svelte';
    import { slide } from 'svelte/transition';
    import { identity as getIdentity, toaster } from '$lib';

    const identity = getIdentity();

    const [meTable] = useTable(tables.me);
    let me = $derived($meTable[0] ?? null);
    const initialUsername = $derived(me?.username ?? '');
    let username = group(() => initialUsername);
    const setUsername = useReducer(reducers.setUsername);

    const saveUsername = () =>
        username
            .withSaving(() => setUsername({ name: username.value }))
            .catch(console.error)
            .finally(username.toggleEditing);

    let verificationData = $state<UserVerification | null | undefined>(undefined);

    const setWarframeId = useReducer(reducers.setWarframeId);

    const initialWarframeId = $derived(verificationData?.warframeId ?? '');
    let warframeId = group(() => initialWarframeId);

    const saveWarframeId = () =>
        warframeId
            .withSaving(() => setWarframeId({ id: warframeId.value }))
            .catch(console.error)
            .finally(warframeId.toggleEditing);

    $effect(() => {
        if (me && !me.verified) {
            const [table] = useTable(tables.verification);

            const unsubscribe = table.subscribe((value) => {
                verificationData = value[0] ?? null;
            });
            return unsubscribe;
        } else {
            verificationData = null;
        }
    });
</script>

<div class="flex flex-1 flex-col items-center justify-center gap-32">
    <Section title="Username">
        <div class="flex gap-2">
            <div class="input-group hover:preset-tonal w-full grid-cols-[auto_1fr]">
                <div class="ig-cell">
                    <User />
                </div>
                <input
                    type="text"
                    class="ig-input"
                    bind:value={username.value}
                    disabled={!username.isEditing}
                />
            </div>
            <div class="flex w-28 gap-2">
                {#if !username.isEditing}
                    <button
                        class="btn-icon preset-filled-surface-900-100 w-full"
                        onclick={username.toggleEditing}
                    >
                        <Pen class="size-6" />
                    </button>
                {:else}
                    <button
                        class="btn-icon preset-filled-success-300-700 w-full"
                        onclick={saveUsername}
                    >
                        {#if username.isSaving}
                            <LoaderCircle class="size-6 animate-spin" />
                        {:else}
                            <Check class="size-6" />
                        {/if}
                    </button>
                    <button
                        class="btn-icon preset-filled-error-300-700 w-full"
                        onclick={() => {
                            username.value = initialUsername;
                            username.toggleEditing();
                        }}
                    >
                        <X class="size-6" />
                    </button>
                {/if}
            </div>
            {#if username.errorText}
                <p class="text-error-300-700 text-xs">{username.errorText}</p>
            {/if}
        </div>

        <div class="flex gap-2">
            <div class="input-group hover:preset-tonal w-full grid-cols-[auto_1fr]">
                <div class="ig-cell">
                    <IdCard />
                </div>
                <input
                    type="text"
                    class="ig-input text-xs"
                    value={identity.toHexString()}
                    disabled
                />
            </div>

            <div class="flex w-20 gap-2">
                <button
                    class="btn-icon preset-filled-surface-900-100 w-full"
                    onclick={() => {
                        navigator.clipboard.writeText(identity.toHexString());
                        toaster.create({
                            title: 'Copied!',
                            type: 'success',
                        });
                    }}
                    title="Copy to clipboard"
                >
                    <Clipboard class="size-6" />
                </button>
            </div>
        </div>
    </Section>

    <Section title="Verification">
        {#if me?.verified}
            <div class="flex items-center gap-2">
                <BadgeCheck class="text-success-500 size-10" />
                <span class="text-success-500 font-bold">Verified</span>
            </div>
        {:else if verificationData === null}
            <div class="flex items-center gap-2">
                <span class="font-bold">Please enter a username first.</span>
            </div>
        {:else if verificationData !== undefined && verificationData !== null}
            <div class="flex w-full flex-col gap-4">
                <div>
                    <span class="label-text">Code</span>
                    <div class="flex gap-2">
                        <div class="input-group w-full grid-cols-[auto_1fr] opacity-80">
                            <div class="ig-cell"><Hash /></div>
                            <input
                                type="text"
                                class="ig-input"
                                value={verificationData?.code ?? 'Loading...'}
                                disabled
                            />
                        </div>

                        <div class="flex w-20 gap-2">
                            <button
                                class="btn-icon preset-filled-surface-900-100 w-full"
                                onclick={() => {
                                    navigator.clipboard.writeText(verificationData?.code ?? '');
                                    toaster.create({
                                        title: 'Copied!',
                                        type: 'success',
                                    });
                                }}
                                title="Copy to clipboard"
                            >
                                <Clipboard class="size-6" />
                            </button>
                        </div>
                    </div>
                </div>

                <div>
                    <span class="label-text">Warframe User ID</span>
                    <div class="flex gap-2">
                        <div class="input-group hover:preset-tonal w-full grid-cols-[auto_1fr]">
                            <div class="ig-cell"><User /></div>
                            <input
                                type="text"
                                placeholder="Enter Warframe ID"
                                class="ig-input"
                                bind:value={warframeId.value}
                                disabled={!warframeId.isEditing}
                            />
                        </div>

                        <div class="flex w-28 gap-2">
                            {#if !warframeId.isEditing}
                                <button
                                    class="btn-icon preset-filled-surface-900-100 w-full"
                                    onclick={warframeId.toggleEditing}
                                >
                                    <Pen class="size-6" />
                                </button>
                            {:else}
                                <button
                                    class="btn-icon preset-filled-success-300-700 w-full"
                                    onclick={saveWarframeId}
                                >
                                    {#if warframeId.isSaving}
                                        <LoaderCircle class="size-6 animate-spin" />
                                    {:else}
                                        <Check class="size-6" />
                                    {/if}
                                </button>
                                <button
                                    class="btn-icon preset-filled-error-300-700 w-full"
                                    onclick={() => {
                                        warframeId.value = initialWarframeId;
                                        warframeId.toggleEditing();
                                    }}
                                >
                                    <X class="size-6" />
                                </button>
                            {/if}
                        </div>
                    </div>
                </div>
                {#if warframeId.errorText}
                    <p class="text-error-300-700 text-xs">{warframeId.errorText}</p>
                {/if}
            </div>
            <hr class="hr mt-2" />
            <Accordion collapsible>
                <Accordion.Item value="1">
                    <h3>
                        <Accordion.ItemTrigger
                            class="flex items-center justify-between gap-2 font-bold"
                        >
                            How it works
                            <Accordion.ItemIndicator class="group">
                                <ChevronDownIcon
                                    class="h-5 w-5 transition group-data-[state=open]:rotate-180"
                                />
                            </Accordion.ItemIndicator>
                        </Accordion.ItemTrigger>
                    </h3>
                    <Accordion.ItemContent>
                        {#snippet element(attributes)}
                            {#if !attributes.hidden}
                                <div {...attributes} transition:slide={{ duration: 150 }}>
                                    <ol class="text-token list-inside list-decimal space-y-4">
                                        <li>
                                            Head to <a
                                                href="https://yareli.net/account"
                                                class="anchor"
                                                target="_blank"
                                                rel="noreferrer"
                                            >
                                                yareli.net
                                            </a>
                                            and follow the instructions to obtain your Warframe User
                                            ID.
                                        </li>

                                        <li>
                                            Edit the <strong>Warframe User ID</strong>
                                            field and paste your obtained ID in there.
                                        </li>
                                        <li>
                                            Copy your unique <strong>Code</strong>
                                            from the field above, open Warframe, and change your current
                                            loadout name to this exact code.
                                        </li>
                                        <li>
                                            Wait. The system verifies usernames every 10 minutes.
                                        </li>
                                    </ol>
                                </div>
                            {/if}
                        {/snippet}
                    </Accordion.ItemContent>
                </Accordion.Item>
            </Accordion>
            <hr class="hr" />
        {/if}
    </Section>
</div>
