<script lang="ts">
    import {
        ArrowRight,
        BadgeCheck,
        ChevronDownIcon,
        Gamepad2,
        Globe,
        Hash,
        IdCard,
        LoaderCircle,
        User as UserIcon,
        XIcon,
    } from 'lucide-svelte';
    import Section from './Section.svelte';
    import { useReducer, useTable } from 'spacetimedb/svelte';
    import { reducers, tables } from '$lib/module_bindings';
    import { group } from '$lib/utils/group.svelte';
    import { Region, VerifyTimer, Permissions, User, Me } from '$lib/module_bindings/types';
    import { Accordion, Dialog, Portal } from '@skeletonlabs/skeleton-svelte';
    import { slide } from 'svelte/transition';
    import {
        identity as getIdentity,
        makeToComboboxData,
        me as getMe,
        preferredRegion,
        toaster,
    } from '$lib';
    import { EditableInput, CopyInput } from '$lib/components';
    import ComboboxInput from '$lib/components/inputs/ComboboxInput.svelte';
    import Countdown from '$lib/components/Countdown.svelte';
    import { Timestamp } from 'spacetimedb';
    import { useClerkContext } from 'svelte-clerk';
    import { Bitmask, UserFlags } from '$lib/utils/bitmask';

    let clerkCtx = useClerkContext();
    let identity = getIdentity();

    let me = getMe();

    let [verifyTimerTable, verifyTimerIsReady] = useTable(tables.my_verify_timer);
    let verifyTimer: VerifyTimer | null = $derived($verifyTimerTable[0] ?? null);

    const initialUsername = $derived(me.current?.user.name ?? '');

    // username edit
    let username = group(() => initialUsername);
    const setUsername = useReducer(reducers.setUsername);

    const saveUsername = () =>
        username
            .withSaving(async () => {
                if (username.value.trim()) await setUsername({ name: username.value });
            })
            .catch((e) => {
                toaster.create({
                    title: 'Error',
                    description: e.message,
                    type: 'error',
                });
                username.value = initialUsername;
                username.isSaving = false;
            })
            .finally(username.toggleEditing);

    const cancelUsername = () => {
        username.value = initialUsername;
        username.toggleEditing();
    };

    const setWarframeId = useReducer(reducers.setWarframeId);

    const initialWarframeId = $derived(verifyTimer?.warframeId ?? '');
    let warframeId = group(() => initialWarframeId);

    const saveWarframeId = () =>
        warframeId
            .withSaving(() => setWarframeId({ warframeId: warframeId.value }))
            .catch((e) => {
                warframeId.isSaving = false;
                console.error(e);
            })
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

    let deleteAccountDialogOpen = $state(false);
    let isAccountDeleting = $state(false);

    const deleteAccountReducer = useReducer(reducers.deleteMyAccount);

    const deleteAccount = async () => {
        isAccountDeleting = true;
        await deleteAccountReducer();
        isAccountDeleting = false;
        deleteAccountDialogOpen = false;
    };
</script>

<svelte:head>
    <title>Account Info</title>
</svelte:head>

<div
    class="mb-8 flex min-h-full flex-1 flex-col items-center justify-center gap-16 pb-4! xl:p-4 xl:pb-8"
>
    <Section title="User Details">
        <div class="flex flex-col gap-4">
            <EditableInput
                label="Warframe Username"
                group={username}
                icon={UserIcon}
                onSave={saveUsername}
                onCancel={cancelUsername}
            />

            <CopyInput
                label="User ID"
                icon={IdCard}
                value={identity.toHexString()}
                inputClass="text-xs"
            />

            <div class="flex flex-col gap-4">
                <CopyInput
                    label="Clerk User ID"
                    icon={IdCard}
                    value={clerkCtx.auth.userId ?? ''}
                    inputClass="text-xs"
                />
                <div class="flex w-full justify-end">
                    <a
                        href="/app/info/clerk"
                        class="btn btn-sm preset-filled-surface-900-100 flex items-center gap-2"
                    >
                        <span>View Clerk Details</span>
                        <ArrowRight />
                    </a>
                </div>
            </div>
        </div>
    </Section>

    <Section title="Verification">
        {#if !me.current}
            <div class="flex items-center gap-2">
                <span class="font-bold">Please enter your warframe username first.</span>
            </div>
        {:else if Bitmask.has(me.current.details.flags.bits, UserFlags.Verified)}
            <div class="flex items-center gap-2">
                <BadgeCheck class="text-success-500 size-10" />
                <span class="text-success-500 text-2xl font-bold">Verified</span>
            </div>
        {:else if $verifyTimerIsReady}
            <div class="flex w-full flex-col gap-4">
                <EditableInput
                    label="Warframe User ID"
                    group={warframeId}
                    icon={Gamepad2}
                    onSave={saveWarframeId}
                    onCancel={cancelWarframeId}
                />

                <CopyInput
                    label="Code"
                    icon={Hash}
                    value={verifyTimer?.code ? verifyTimer.code : 'Enter Warframe User ID first...'}
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
                                                from the field above, open Warframe, and change your current
                                                loadout name to this exact code.
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

    <Section title="Danger Zone">
        <Dialog
            open={deleteAccountDialogOpen}
            onOpenChange={(e) => (deleteAccountDialogOpen = e.open)}
        >
            <Dialog.Trigger
                class="btn preset-filled-error-300-700 h-full self-start"
                disabled={me === null}
            >
                Delete Account
            </Dialog.Trigger>
            <Portal>
                <Dialog.Backdrop class="bg-surface-50-950/50 fixed inset-0 z-50" />
                <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center px-2">
                    <Dialog.Content class="card bg-surface-100-900 w-lg space-y-4 p-8 shadow-xl ">
                        <header class="flex items-center justify-between">
                            <Dialog.Title class="text-lg font-bold">Account Deletion</Dialog.Title>
                            <Dialog.CloseTrigger class="btn-icon hover:preset-tonal">
                                <XIcon class="size-4" />
                            </Dialog.CloseTrigger>
                        </header>
                        <Dialog.Description>
                            Are you sure you want to delete your account?
                        </Dialog.Description>
                        <footer class="flex justify-end gap-2">
                            <Dialog.CloseTrigger class="btn preset-tonal">
                                Cancel
                            </Dialog.CloseTrigger>
                            <button
                                type="button"
                                class="btn preset-filled-error-300-700"
                                onclick={deleteAccount}
                            >
                                {#if isAccountDeleting}
                                    <LoaderCircle class="animate-spin" />
                                {:else}
                                    Yes
                                {/if}
                            </button>
                        </footer>
                    </Dialog.Content>
                </Dialog.Positioner>
            </Portal>
        </Dialog>
    </Section>
</div>
