<script lang="ts">
    import { Modal, Slider } from '@skeletonlabs/skeleton-svelte';
    import type { Relic } from '$lib/types/relic';
    import Combobox from './Combobox.svelte';
    import type { LobbyAndUser } from '$lib/types/lobby';
    import FormGroup from './FormGroup.svelte';
    import LobbyCreateModal from './modals/LobbyCreateModalBody.svelte';

    interface Props {
        relics: Relic[];
        onLobbyCreate?: (lobby: LobbyAndUser) => void;
    }

    let { relics, onLobbyCreate }: Props = $props();

    let showModal = $state(false);
    const close = () => (showModal = false);
</script>

<Modal
    bind:open={showModal}
    triggerBase="btn preset-filled-success-200-800 h-full"
    contentBase="card bg-surface-100-900 p-8 space-y-4 shadow-xl max-w-screen-sm "
    backdropClasses="backdrop-blur-sm !z-30"
    positionerZIndex="!z-40"
    preventScroll={false}
>
    {#snippet trigger()}
        Create Lobby
    {/snippet}

    {#snippet content()}
        <LobbyCreateModal {relics} {onLobbyCreate} bind:open={showModal} />
    {/snippet}
</Modal>
