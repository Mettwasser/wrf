<script lang="ts">
    import {
        BadgeCheck,
        ChevronDownIcon,
        Gamepad2,
        Globe,
        Hash,
        IdCard,
        User as UserIcon,
    } from 'lucide-svelte';
    import Section from './Section.svelte';
    import { useReducer, useTable } from 'spacetimedb/svelte';
    import { reducers, tables } from '$lib/module_bindings';
    import { group } from '$lib/utils/group.svelte';
    import { Region, VerifyTimer, User, UserWarframeId } from '$lib/module_bindings/types';
    import { Accordion } from '@skeletonlabs/skeleton-svelte';
    import { slide } from 'svelte/transition';
    import { identity as getIdentity, makeToComboboxData, preferredRegion } from '$lib';
    import { EditableInput, CopyInput } from '$lib/components';
    import ComboboxInput from '$lib/components/inputs/ComboboxInput.svelte';
    import Countdown from '$lib/components/Countdown.svelte';
    import { Timestamp } from 'spacetimedb';

    const identity = getIdentity();

    // subscriptions
    const [meTable] = useTable(tables.me);
    let me: User | null = $derived($meTable[0] ?? null);

    let [warframeIdTable, warframeIdIsReady] = useTable(tables.warframe_id);
    let fetchedWarframeId: UserWarframeId | null = $derived($warframeIdTable[0] ?? null);

    let [verifyTimerTable, verifyTimerIsReady] = useTable(tables.my_verify_timer);
    let verifyTimer: VerifyTimer | null = $derived($verifyTimerTable[0] ?? null);
    $inspect($verifyTimerTable).with((type, val) => console.log('table: ', val));
    $inspect(verifyTimer).with((type, val) => console.log('derived: ', val));

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

    const setWarframeId = useReducer(reducers.setWarframeId);

    const initialWarframeId = $derived(fetchedWarframeId?.warframeId ?? '');
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
</script>

<div class="flex min-h-full flex-1 flex-col items-center justify-center gap-16 p-4">
    <Section title="Username">
        <div class="flex flex-col gap-4">
            <EditableInput
                group={username}
                icon={UserIcon}
                onSave={saveUsername}
                onCancel={cancelUsername}
            />

            <CopyInput icon={IdCard} value={identity.toHexString()} inputClass="text-xs" />
        </div>
    </Section>

    <Section title="Verification">
        {#if !me}
            <div class="flex items-center gap-2">
                <span class="font-bold">Please enter a username first.</span>
            </div>
        {:else if me.verified}
            <div class="flex items-center gap-2">
                <BadgeCheck class="text-success-500 size-10" />
                <span class="text-success-500 text-2xl font-bold">Verified</span>
            </div>
        {:else if $warframeIdIsReady && $verifyTimerIsReady}
            <div class="flex w-full flex-col gap-4">
                <CopyInput
                    label="Code"
                    icon={Hash}
                    value={verifyTimer?.code
                        ? verifyTimer.code
                        : $warframeIdIsReady
                          ? 'Enter Warframe User ID first...'
                          : 'Loading...'}
                />

                <EditableInput
                    label="Warframe User ID"
                    group={warframeId}
                    icon={Gamepad2}
                    onSave={saveWarframeId}
                    onCancel={cancelWarframeId}
                />

                {#if verifyTimer}
                    <Countdown
                        targetTimestamp={Number(
                            (verifyTimer.scheduledAt.value as Timestamp).toMillis()
                        )}
                    >
                        {#snippet content({ formattedTime, remainingMs })}
                            {#if remainingMs > 0}
                                <div
                                    class="card preset-outlined border-surface-200-800 bg-surface-300-700/15 flex gap-2 p-4"
                                >
                                    <span>Next Verification attempt in:</span>
                                    <span class="badge preset-filled-primary-300-700 font-semibold">
                                        {formattedTime}
                                    </span>
                                </div>
                            {/if}
                        {/snippet}
                    </Countdown>
                {/if}
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
                                                Wait. The system tries to verify your username after
                                                10 minutes.
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
    <!-- <hr class="hr -my-16 w-1/3" /> -->
    <Section title="Preferences">
        <ComboboxInput
            label="Region"
            data={makeToComboboxData(Object.keys(Region.variants))}
            displayAsUppercase
            placeholder="Region"
            group={region}
            icon={Globe}
            onSave={onRegionSave}
            onCancel={onRegionCancel}
        />
    </Section>
</div>
