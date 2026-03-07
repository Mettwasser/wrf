<script lang="ts">
    import { makeToComboboxData } from '$lib';
    import { Region, RelicRefinement, RotationType } from '$lib/module_bindings/types';
    import Combobox from '../Combobox.svelte';
    import Info from 'lucide-svelte/icons/info';
    import Tooltip from '../Tooltip.svelte';

    interface Props {
        open: boolean;
        eraFilter: string[];
        refinementFilter: RelicRefinement['tag'][];
        regionFilter: Region['tag'][];
        only2A2B: boolean | undefined;
    }

    let {
        open = $bindable(),
        eraFilter = $bindable(),
        refinementFilter = $bindable(),
        regionFilter = $bindable(),
        only2A2B = $bindable(undefined),
    }: Props = $props();

    const close = () => (open = false);
    const clearFilters = () => {
        eraFilter = [];
        refinementFilter = [];
        regionFilter = [];
        only2A2B = undefined;
    };
</script>

{#snippet values<T>(items: T[], uppercase?: boolean)}
    <div class="mt-2 flex flex-wrap gap-2">
        {#each items as item (item)}
            <span class="badge preset-filled bg-surface-950-50/80 {uppercase ? 'uppercase' : ''}">
                {item}
            </span>
        {:else}
            <span class="badge invisible">Empty</span>
        {/each}
    </div>
{/snippet}

<div class="flex flex-col gap-8">
    <header class="xsm:flex-row max-xsm:gap-4 flex flex-col items-center justify-between">
        <h2 class="h2">Filters</h2>
        <button class="btn preset-tonal max-xsm:w-full" onclick={clearFilters}>Clear All</button>
    </header>

    <div class="flex flex-col gap-6">
        <div>
            <span class="label-text">Relic Era</span>
            <Combobox
                data={makeToComboboxData(['Lith', 'Meso', 'Neo', 'Axi'])}
                bind:value={eraFilter}
                placeholder="All Eras"
                alternatePlaceHolder="Choose more"
                multiple
            />
            {@render values(eraFilter)}
        </div>

        <div>
            <span class="label-text">Relic Refinement</span>
            <Combobox
                data={makeToComboboxData(Object.keys(RelicRefinement.variants))}
                bind:value={refinementFilter}
                placeholder="All Refinements"
                alternatePlaceHolder="Choose more"
                multiple
            />
            {@render values(refinementFilter)}
        </div>

        <div>
            <span class="label-text">Region</span>
            <Combobox
                data={makeToComboboxData(Object.keys(Region.variants))}
                bind:value={regionFilter}
                placeholder="All Regions"
                alternatePlaceHolder="Choose more"
                displayAsUppercase
                multiple
            />
            {@render values(regionFilter, true)}
        </div>

        <div>
            <span class="label-text">Rotation</span>
            <div class="card preset-filled-surface-200-800 flex grow-0 flex-col gap-2 p-3">
                <div class="flex items-center space-x-2">
                    <input
                        id="rotation-1"
                        class="radio"
                        type="radio"
                        name="rotation"
                        value={undefined}
                        bind:group={only2A2B}
                    />
                    <label for="rotation-1">Any</label>
                </div>
                <div class="flex items-center space-x-2">
                    <input
                        id="rotation-2"
                        class="radio"
                        type="radio"
                        name="rotation"
                        value={false}
                        bind:group={only2A2B}
                    />
                    <label for="rotation-2">Default</label>
                </div>
                <div class="flex items-center space-x-2">
                    <input
                        id="rotation-3"
                        class="radio"
                        type="radio"
                        name="rotation"
                        value={true}
                        bind:group={only2A2B}
                    />
                    <label for="rotation-3">2A2B</label>
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
            </div>
        </div>
    </div>

    <footer class="mt-4 flex justify-end gap-4">
        <button type="button" class="btn preset-filled-primary-300-700 w-full" onclick={close}>
            Show Results
        </button>
    </footer>
</div>
