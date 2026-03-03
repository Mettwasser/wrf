<script lang="ts">
    import {
        BadgeCheck,
        ChevronDownIcon,
        Gamepad2,
        Globe,
        Hash,
        IdCard,
        User,
    } from 'lucide-svelte';
    import Section from './Section.svelte';
    import { useReducer, useTable } from 'spacetimedb/svelte';
    import { reducers, tables } from '$lib/module_bindings';
    import { group } from '$lib/utils/group.svelte';
    import type { UserVerification } from '$lib/module_bindings/types';
    import { Accordion } from '@skeletonlabs/skeleton-svelte';
    import { slide } from 'svelte/transition';
    import { identity as getIdentity, preferredRegion } from '$lib';
    import { EditableInput, CopyInput } from '$lib/components';

    const identity = getIdentity();

    // subscriptions
    const [meTable] = useTable(tables.me);
    let me = $derived($meTable[0] ?? null);
    const initialUsername = $derived(me?.username ?? '');

    // username edit
    let username = group(() => initialUsername);
    const setUsername = useReducer(reducers.setUsername);

    const saveUsername = () =>
        username
            .withSaving(() => setUsername({ name: username.value }))
            .catch(console.error)
            .finally(username.toggleEditing);

    const cancelUsername = () => {
        username.value = initialUsername;
        username.toggleEditing();
    };

    // verification
    let verificationData = $state<UserVerification | null | undefined>(undefined);

    const setWarframeId = useReducer(reducers.setWarframeId);

    const initialWarframeId = $derived(verificationData?.warframeId ?? '');
    let warframeId = group(() => initialWarframeId);

    const saveWarframeId = () =>
        warframeId
            .withSaving(() => setWarframeId({ id: warframeId.value }))
            .catch(console.error)
            .finally(warframeId.toggleEditing);

    const cancelWarframeId = () => {
        warframeId.value = initialWarframeId;
        warframeId.toggleEditing();
    };

    // preferences
    const region = group(() => preferredRegion.current);
    const onRegionSave = () => {
        preferredRegion.current = region.value;
        region.toggleEditing();
    };
    const onRegionCancel = () => {
        region.value = preferredRegion.current;
        region.toggleEditing();
    };

    // hooks
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

<div class="flex flex-1 flex-col items-center justify-center gap-24">
    <Section title="Username">
        <div class="flex flex-col gap-4">
            <EditableInput
                group={username}
                icon={User}
                onSave={saveUsername}
                onCancel={cancelUsername}
            />

            <CopyInput icon={IdCard} value={identity.toHexString()} inputClass="text-xs" />
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
                <CopyInput
                    label="Code"
                    icon={Hash}
                    value={verificationData?.code ?? 'Loading...'}
                />

                <EditableInput
                    label="Warframe User ID"
                    group={warframeId}
                    icon={Gamepad2}
                    onSave={saveWarframeId}
                    onCancel={cancelWarframeId}
                />
            </div>

            <!-- Help Section for verifications -->
            <div class="flex flex-col gap-2">
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
                                                from the field above, open Warframe, and change your
                                                current loadout name to this exact code.
                                            </li>
                                            <li>
                                                Wait. The system verifies usernames every 10
                                                minutes.
                                            </li>
                                        </ol>
                                    </div>
                                {/if}
                            {/snippet}
                        </Accordion.ItemContent>
                    </Accordion.Item>
                </Accordion>
                <hr class="hr" />
            </div>
        {/if}
    </Section>
    <hr class="hr w-1/3" />
    <Section title="Preferences">
        <EditableInput
            group={region}
            icon={Globe}
            onSave={onRegionSave}
            onCancel={onRegionCancel}
        />
    </Section>
</div>
