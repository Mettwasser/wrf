<script lang="ts">
    import { REGIONS, type LobbyAndUser } from '$lib/types/lobby';
    import type { Relic } from '$lib/types/relic';
    import { Slider } from '@skeletonlabs/skeleton-svelte';
    import Info from 'lucide-svelte/icons/info';

    import FormGroup from '../FormGroup.svelte';
    import { ErrorCollection, refreshError } from '$lib/utils/error_collection.svelte';
    import { makeToComboboxData } from '$lib';
    import Combobox from '../Combobox.svelte';

    interface Props {
        relics: Relic[];
        onLobbyCreate?: (lobby: LobbyAndUser) => void;
        open: boolean;
    }

    let { relics, onLobbyCreate, open = $bindable() }: Props = $props();
    const close = () => (open = false);

    const mappedRelics = relics.map((relic) => `${relic.era} ${relic.category}`);

    let lobbySize = $state([4]);
    let activity = $state(['']);
    let refinement = $state(['']);
    let region = $state(['']);

    let errorCollection = new ErrorCollection();

    const activityEmptyId = 1;
    const refinementEmptyId = 2;
    const regionEmptyId = 3;

    $effect(() => {
        refreshError(activity, activityEmptyId, 'Please select a relic', errorCollection);
    });

    $effect(() => {
        refreshError(
            refinement,
            refinementEmptyId,
            "Please choose the relic's refinement",
            errorCollection
        );
    });
    $effect(() => {
        refreshError(
            region,
            regionEmptyId,
            "Please choose the Region you're playing in",
            errorCollection
        );
    });
</script>

<div class="flex flex-col gap-8">
    <form class="flex flex-col gap-6">
        <header class="flex justify-between">
            <h2 class="h2">Create a Lobby</h2>
        </header>

        <FormGroup title="Select a Relic" errorIds={[activityEmptyId]} {errorCollection}>
            <Combobox
                data={makeToComboboxData(mappedRelics)}
                limit={5}
                bind:value={activity}
                placeholder="Select a relic..."
            />
        </FormGroup>
        <FormGroup title="Relic Refinement" errorIds={[refinementEmptyId]} {errorCollection}>
            <Combobox
                data={makeToComboboxData(['Intact', 'Exceptional', 'Flawless', 'Radiant'])}
                bind:value={refinement}
                placeholder="Relic Refinement..."
            />
        </FormGroup>
        <FormGroup title="Region" errorIds={[regionEmptyId]} {errorCollection}>
            <Combobox
                data={makeToComboboxData(Object.keys(REGIONS))}
                bind:value={region}
                placeholder="Region..."
            />
        </FormGroup>
        <div class="label">
            <span class="label-text flex items-center gap-1">
                Lobby Size <Info class="size-4" />
            </span>
            <Slider
                min={2}
                max={4}
                value={lobbySize}
                onValueChange={(e) => (lobbySize = e.value)}
                class="mt-2!"
            >
                <Slider.Control>
                    <Slider.Track>
                        <Slider.Range class="bg-primary-400-600" />
                    </Slider.Track>
                    <Slider.Thumb index={0} class="ring-primary-400-600" />
                </Slider.Control>
                <Slider.MarkerGroup>
                    {#each [2, 3, 4] as m}
                        <Slider.Marker value={m}>{m}</Slider.Marker>
                    {/each}
                </Slider.MarkerGroup>
            </Slider>
        </div>

        <footer class="mt-4 flex justify-end gap-4">
            <button type="button" class="btn preset-tonal" onclick={close}>Cancel</button>
            <button
                type="submit"
                class="btn preset-filled-primary-300-700"
                disabled={errorCollection.hasErrors()}
            >
                Confirm
            </button>
        </footer>
    </form>
</div>
