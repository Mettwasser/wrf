<script lang="ts">
    import { Modal } from '@skeletonlabs/skeleton-svelte';
    import type { Relic } from '$lib/types/relic';
    import Combobox from './Combobox.svelte';

    interface ComboboxData {
        label: string;
        value: string;
    }

    let { relics }: { relics: Relic[] } = $props();

    const comboboxData: ComboboxData[] = relics.map((relic) => ({
        label: `${relic.era} ${relic.category}`,
        value: `${relic.era} ${relic.category}`,
    }));

    let activity = $state('');

    let showModal = $state(false);

    const close = () => (showModal = false);
    function onSubmit(e: SubmitEvent) {
        e.preventDefault();
    }
</script>

<Modal
    bind:open={showModal}
    triggerBase="btn preset-filled-success-200-800 h-full"
    contentBase="card bg-surface-100-900 p-8 space-y-4 shadow-xl max-w-screen-sm"
    backdropClasses="backdrop-blur-sm !z-30"
    positionerZIndex="!z-40"
    preventScroll={false}
>
    {#snippet trigger()}
        Create Lobby
    {/snippet}

    {#snippet content()}
        <form onsubmit={onSubmit}>
            <div class="flex flex-col gap-8">
                <header class="flex justify-between">
                    <h2 class="h2">Create a Lobby</h2>
                </header>

                <label class="label">
                    <span class="label-text">Select a Relic</span>

                    <Combobox
                        data={comboboxData}
                        limit={5}
                        searchThreshold={0.5}
                        onInputChange={(s) => (activity = s)}
                        placeholder="Select a relic..."
                    />
                </label>
                <footer class="flex justify-end gap-4">
                    <button type="button" class="btn preset-tonal" onclick={close}>Cancel</button>
                    <button type="button" class="btn preset-filled-primary-300-700" onclick={close}>
                        Confirm
                    </button>
                </footer>
            </div>
        </form>
    {/snippet}
</Modal>
