<script lang="ts">
    import { REGIONS, type Lobby, type LobbyAndUser } from '$lib/types/lobby';
    import type { Relic } from '$lib/types/relic';
    import { Slider, type ToastContext } from '@skeletonlabs/skeleton-svelte';
    import Combobox from '../Combobox.svelte';
    import Info from 'lucide-svelte/icons/info';

    import FormGroup from '../FormGroup.svelte';
    import { ErrorCollection, refreshError } from '$lib/utils/error_collection.svelte';
    import { fetch, makeToComboboxData, URL } from '$lib';
    import { getContext } from 'svelte';
    import Loader from 'lucide-svelte/icons/loader-circle';

    interface Props {
        relics: Relic[];
        onLobbyCreate?: (lobby: LobbyAndUser) => void;
        open: boolean;
    }

    const toast: ToastContext = getContext('toast');

    let { relics, onLobbyCreate, open = $bindable() }: Props = $props();
    const close = () => (open = false);

    let lobbySize = $state([4]);
    let activity = $state(['']);
    let refinement = $state(['']);
    let region = $state(['']);

    let errorCollection = new ErrorCollection();

    const activityId = 1;
    const refinementId = 2;
    const regionId = 3;

    let response = $state<Promise<Response | void> | undefined>();

    async function onSubmit(e: SubmitEvent) {
        e.preventDefault();

        response = fetch(`${URL}/api/lobbies`, {
            method: 'POST',
            body: JSON.stringify({
                lobbySize: lobbySize[0],
                activity: activity[0],
                refinement: refinement[0],
                region: region[0],
            }),
        }).then(async (r) => {
            switch (r.status) {
                case 409:
                    toast.create({
                        title: 'Failed',
                        description: 'You are already hosting a lobby!',
                        type: 'error',
                    });
                    break;

                case 200:
                    let lobby: LobbyAndUser = await r.json();
                    onLobbyCreate?.(lobby);
                    // TODO: redirect to lobby view
                    close();
                    break;

                default:
                    break;
            }
        });
    }

    $effect(() => {
        refreshError(activity, activityId, 'Please select a relic', errorCollection);

        refreshError(
            refinement,
            refinementId,
            "Please choose the relic's refinement",
            errorCollection
        );

        refreshError(
            region,
            regionId,
            "Please choose the Region you're playing in",
            errorCollection
        );
    });
</script>

<div class="flex flex-col gap-8">
    <form onsubmit={onSubmit} class="flex flex-col gap-6">
        <header class="flex justify-between">
            <h2 class="h2">Create a Lobby</h2>
        </header>

        <FormGroup title="Select a Relic" errorId={activityId} {errorCollection}>
            <Combobox
                data={makeToComboboxData(relics.map((relic) => `${relic.era} ${relic.category}`))}
                limit={5}
                bind:value={activity}
                placeholder="Select a relic..."
            />
        </FormGroup>
        <FormGroup title="Relic Refinement" errorId={refinementId} {errorCollection}>
            <Combobox
                data={makeToComboboxData(['Intact', 'Exceptional', 'Flawless', 'Radiant'])}
                bind:value={refinement}
                placeholder="Relic Refinement..."
            />
        </FormGroup>

        <FormGroup title="Region" errorId={regionId} {errorCollection}>
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
                bind:value={lobbySize}
                meterBg="bg-primary-400-600"
                thumbRingColor="ring-primary-400-600"
                markers={[2, 3, 4]}
                base="!mt-2"
            />
        </div>

        <footer class="mt-4 flex justify-end gap-4">
            <button type="button" class="btn preset-tonal" onclick={close}>Cancel</button>
            <button
                type="submit"
                class="btn preset-filled-primary-300-700"
                disabled={errorCollection.hasErrors() && response === undefined}
            >
                {#if response}
                    {#await response}
                        <Loader class="animate-spin" />
                    {:then}
                        ...
                    {/await}
                {/if}
                Confirm
            </button>
        </footer>
    </form>
</div>
