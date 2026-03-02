<script lang="ts">
    import { Dialog, Portal } from '@skeletonlabs/skeleton-svelte';
    import type { Relic } from '$lib/types/relic';
    import LobbyCreateModal from './modals/LobbyCreateModalBody.svelte';
    import type { LobbyAndUser } from '$lib/types/lobby_and_user';

    interface Props {
        relics: Relic[];
    }

    let { relics }: Props = $props();

    let showModal = $state(false);
</script>

<Dialog open={showModal} onOpenChange={(e) => (showModal = e.open)}>
    <Dialog.Trigger class="btn preset-filled-success-200-800 h-full">Create Lobby</Dialog.Trigger>
    <Portal>
        <Dialog.Backdrop class="bg-surface-50-950/50 fixed inset-0 z-50" />
        <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center">
            <Dialog.Content class="card bg-surface-100-900 w-lg space-y-4 p-8 shadow-xl ">
                <LobbyCreateModal {relics} bind:open={showModal} />
            </Dialog.Content>
        </Dialog.Positioner>
    </Portal>
</Dialog>
