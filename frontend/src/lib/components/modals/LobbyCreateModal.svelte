<script lang="ts">
    import type { Lobby, RelicRefinement } from '$lib/types/lobby';
    import type { Relic } from '$lib/types/relic';
    import { Slider } from '@skeletonlabs/skeleton-svelte';
    import Combobox from '../Combobox.svelte';
    import Info from 'lucide-svelte/icons/info';

    import FormGroup from '../FormGroup.svelte';
    import { ErrorCollection } from '$lib/utils/error_collection.svelte';
    import { makeToComboboxData } from '$lib';
    import { untrack } from 'svelte';

    interface Props {
        relics: Relic[];
        onLobbyCreate?: (lobby: Lobby) => void;
    }

    let { relics, onLobbyCreate }: Props = $props();

    const comboboxData = makeToComboboxData(
        relics.map((relic) => `${relic.era} ${relic.category}`)
    );

    let errorCollection = new ErrorCollection();

    let lobbySize = $state([4]);
    let activity = $state(['']);
    let refinement = $state(['']);

    const activityId = 1;
    const refinementId = 2;

    async function onSubmit(e: SubmitEvent) {
        e.preventDefault();
        onLobbyCreate?.();
    }

    $effect(() => {
        if (activity[0] === '')
            untrack(() => errorCollection.addError(activityId, 'Please select a relic'));
        else untrack(() => errorCollection.removeError(activityId));

        if (refinement[0] === '')
            untrack(() =>
                errorCollection.addError(refinementId, "Please choose the relic's refinement")
            );
        else untrack(() => errorCollection.removeError(refinementId));
    });
</script>

<div class="flex flex-col gap-8">
    <header class="flex justify-between">
        <h2 class="h2">Create a Lobby</h2>
    </header>

    <form onsubmit={onSubmit} class="flex flex-col gap-6">
        <FormGroup title="Select a Relic" errorId={activityId} bind:errorCollection>
            <Combobox
                data={comboboxData}
                limit={5}
                bind:value={activity}
                placeholder="Select a relic..."
            />
        </FormGroup>
        <FormGroup title="Relic Refinement" errorId={refinementId} bind:errorCollection>
            <Combobox
                data={makeToComboboxData(['Intact', 'Exceptional', 'Flawless', 'Radiant'])}
                bind:value={refinement}
                placeholder="Relic Refinement..."
            />
        </FormGroup>

        <div class="label">
            <span class="label-text flex items-center gap-1">
                Lobby Size <Info class="size-4" />
            </span>
            <Slider
                min={2}
                max={4}
                bind:value={lobbySize}
                meterBg="bg-primary-400-600"
                thumbRingColor="ring-primary-400-600"
                markers={[2, 3, 4]}
                base="!mt-2"
            />
        </div>
    </form>

    <footer class="mt-4 flex justify-end gap-4">
        <button type="button" class="btn preset-tonal" onclick={close}>Cancel</button>
        <button
            type="button"
            class="btn preset-filled-primary-300-700"
            onclick={close}
            disabled={errorCollection.hasErrors()}
        >
            Confirm
        </button>
    </footer>
</div>
