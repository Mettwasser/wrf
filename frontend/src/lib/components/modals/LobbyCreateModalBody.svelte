<script lang="ts">
    import type { Relic } from '$lib/types/relic';
    import { SegmentedControl } from '@skeletonlabs/skeleton-svelte';
    import Info from 'lucide-svelte/icons/info';

    import FormGroup from '../FormGroup.svelte';
    import { ErrorCollection, refreshError } from '$lib/utils/error_collection.svelte';
    import { makeToComboboxData, toaster } from '$lib';
    import Combobox from '../Combobox.svelte';
    import { Region, RelicRefinement, RotationType } from '$lib/module_bindings/types';
    import Tooltip from '../Tooltip.svelte';
    import { useReducer } from 'spacetimedb/svelte';
    import { reducers } from '$lib/module_bindings';
    import { LoaderCircle } from 'lucide-svelte';

    interface Props {
        relics: Relic[];
        open: boolean;
    }

    let { relics, open = $bindable() }: Props = $props();
    const close = () => (open = false);
    const createLobby = useReducer(reducers.createLobby);

    let relicEra = $state(['']);

    let validRelicCategories = $derived(
        makeToComboboxData(
            relics.filter((relic) => relic.era === relicEra[0]).map((relic) => relic.category)
        )
    );
    let relicCategory = $state(['']);

    let lobbySize = $state(4);
    let refinement: RelicRefinement['tag'][] = $state(['Radiant']);
    // @ts-ignore validated by the form
    let region: Region['tag'][] = $state(['']);
    let twoATwoB = $state(false);

    let errorCollection = new ErrorCollection();

    const relicEraEmptyId = 1;
    const relicCategoryEmptyId = 2;
    const refinementEmptyId = 3;
    const regionEmptyId = 4;

    function watchError(value: () => string[], emptyId: number, message: string) {
        $effect(() => {
            refreshError(value(), emptyId, message, errorCollection);
        });
    }

    // thanks craig!
    watchError(() => relicEra, relicEraEmptyId, 'Please select a Relic Era');
    watchError(() => relicCategory, relicCategoryEmptyId, 'Please select a Relic Category');
    watchError(() => refinement, refinementEmptyId, "Please choose the relic's refinement");
    watchError(() => region, regionEmptyId, "Please choose the Region you're playing in");

    let isSubmitting = $state(false);

    const onsubmit = async (e: SubmitEvent) => {
        e.preventDefault();

        isSubmitting = true;
        try {
            await createLobby({
                activity: `${relicEra[0]} ${relicCategory[0]}`,
                refinement: { tag: refinement[0] },
                region: { tag: region[0] },
                rotationType: twoATwoB ? RotationType.TwoATwoB : RotationType.FourA,
                space: lobbySize,
            });
        } catch (e) {
            toaster.create({
                title: 'Error',
                description: e,
                type: 'error',
            });
        } finally {
            isSubmitting = false;
        }

        open = false;
    };
</script>

<div class="flex flex-col gap-8">
    <form class="flex flex-col gap-6" {onsubmit}>
        <header class="flex justify-between">
            <h2 class="h2">Create a Lobby</h2>
        </header>

        <div>
            <span class="label-text">Relic</span>
            <div class="flex gap-4">
                <FormGroup errorId={relicEraEmptyId} {errorCollection}>
                    <Combobox
                        data={makeToComboboxData(['Lith', 'Meso', 'Neo', 'Axi'])}
                        limit={5}
                        bind:value={relicEra}
                        placeholder="Era..."
                    />
                </FormGroup>

                <FormGroup errorId={relicCategoryEmptyId} {errorCollection}>
                    <Combobox
                        data={validRelicCategories}
                        limit={5}
                        bind:value={relicCategory}
                        placeholder="Category..."
                    />
                </FormGroup>
            </div>
        </div>

        <FormGroup title="Relic Refinement" errorId={refinementEmptyId} {errorCollection}>
            <Combobox
                data={makeToComboboxData(Object.keys(RelicRefinement.variants))}
                bind:value={refinement}
                placeholder="Relic Refinement..."
            />
        </FormGroup>

        <FormGroup title="Region" errorId={regionEmptyId} {errorCollection}>
            <Combobox
                data={makeToComboboxData(Object.keys(Region.variants))}
                bind:value={region}
                placeholder="Region..."
                displayAsUppercase
            />
        </FormGroup>

        <div class="flex items-center space-x-2">
            <input class="checkbox" type="checkbox" bind:checked={twoATwoB} />
            <p>2A2B</p>

            <div class="flex h-full gap-2">
                <div class="border-surface-500 h-4 border-l"></div>
                <Tooltip placement="top" options={{ openDelay: 200 }}>
                    {#snippet text()}
                        <div class="flex w-80 flex-col gap-2">
                            <strong>2A2B: Limits targeted relics to 2 per run</strong>
                            <hr class="hr border-surface-500 w-full" />
                            This lowers the odds of getting the important item multiple times.
                        </div>
                    {/snippet}
                    <Info class="size-4" />
                </Tooltip>
            </div>
        </div>

        <div class="label">
            <span class="label-text flex items-center gap-1">Lobby Size</span>
            <SegmentedControl
                value={lobbySize.toString()}
                onValueChange={(details) => (lobbySize = Number(details.value))}
            >
                <SegmentedControl.Control class="p-0">
                    <SegmentedControl.Indicator class="bg-surface-800-200 outline-none!" />
                    <SegmentedControl.Item value="4">
                        <SegmentedControl.ItemText>4</SegmentedControl.ItemText>
                        <SegmentedControl.ItemHiddenInput />
                    </SegmentedControl.Item>
                    <SegmentedControl.Item value="3">
                        <SegmentedControl.ItemText>3</SegmentedControl.ItemText>
                        <SegmentedControl.ItemHiddenInput />
                    </SegmentedControl.Item>
                    <SegmentedControl.Item value="2">
                        <SegmentedControl.ItemText>2</SegmentedControl.ItemText>
                        <SegmentedControl.ItemHiddenInput />
                    </SegmentedControl.Item>
                </SegmentedControl.Control>
            </SegmentedControl>
        </div>

        <footer class="mt-4 flex justify-end gap-4">
            <button type="button" class="btn preset-tonal" onclick={close}>Cancel</button>
            <button
                type="submit"
                class="btn preset-filled-primary-300-700"
                disabled={errorCollection.hasErrors()}
            >
                {#if isSubmitting}
                    <LoaderCircle class="animate-spin" />
                {:else}
                    Confirm
                {/if}
            </button>
        </footer>
    </form>
</div>
