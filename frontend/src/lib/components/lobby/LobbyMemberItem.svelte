<script lang="ts">
    import { User as UserIcon, Crown, LoaderCircle, UserMinus, Gavel } from 'lucide-svelte';
    import { UserDetails, UserFlags, type User } from '$lib/module_bindings/types';
    import VerifiedBadge from '$lib/components/VerifiedBadge.svelte';
    import type { FullUser } from '$lib/types/full_user';

    interface Props {
        user: FullUser;
        userIsHost: boolean;
        isMe: boolean;
        isHost: boolean;
        isKicking: boolean;
        isBanning: boolean;
        onKick: (user: User) => void;
        onBan: (user: User) => void;
    }

    let { user, userIsHost, isMe, isHost, isKicking, isBanning, onKick, onBan }: Props = $props();
</script>

<div
    class="card bg-surface-300-700/30 flex flex-col justify-between gap-4 p-5 shadow-md transition-all sm:flex-row sm:items-center
    {userIsHost
        ? 'border-primary-600-400/30 border-2'
        : isMe
          ? 'border-success-600-400/30 border-2'
          : 'border-surface-600-400/30 border-2'}"
>
    <div class="flex items-center gap-5">
        <div class="relative max-sm:hidden">
            <div
                class="bg-surface-300-700/50 flex h-14 w-14 items-center justify-center rounded-full shadow-inner"
            >
                <UserIcon size={28} class="opacity-80" />
            </div>
            {#if userIsHost}
                <div
                    class="bg-primary-600-400 absolute -top-1 -right-1 rounded-full p-1.5 shadow-lg"
                >
                    <Crown size={14} class="text-surface-contrast-600-400" />
                </div>
            {/if}
        </div>

        <div class="flex flex-col">
            <div class="flex items-center gap-2">
                <span
                    class="text-2xl font-bold wrap-anywhere {userIsHost
                        ? 'text-primary-300'
                        : isMe
                          ? 'text-success-300/75'
                          : 'text-surface-100'}"
                >
                    {user.name}
                </span>
                <VerifiedBadge flags={{ bits: user.flags }} />
            </div>
            <div class="flex items-center gap-2">
                {#if userIsHost}
                    <span class="text-primary-300/50 text-xs font-bold tracking-widest uppercase">
                        Lobby Host
                    </span>
                {:else if isMe}
                    <span
                        class="text-success-300/60 text-xs font-bold tracking-widest uppercase opacity-50"
                    >
                        You
                    </span>
                {:else}
                    <span class="text-xs font-bold tracking-widest uppercase opacity-50">
                        Member
                    </span>
                {/if}
            </div>
        </div>
    </div>

    <div class={['flex items-center gap-2', !(isHost && !userIsHost) && 'max-sm:hidden']}>
        {#if isHost && !userIsHost}
            <button
                class="btn-icon preset-filled-error-300-700 max-xsm:w-full"
                title="Kick Player"
                onclick={() => onKick(user)}
            >
                {#if isKicking}
                    <LoaderCircle size={20} class="mr-2 animate-spin" />
                {:else}
                    <UserMinus />
                {/if}
            </button>
            <button
                class="btn-icon preset-filled-error-300-700 max-xsm:w-full"
                title="Ban Player"
                onclick={() => onBan(user)}
            >
                {#if isBanning}
                    <LoaderCircle size={20} class="mr-2 animate-spin" />
                {:else}
                    <Gavel />
                {/if}
            </button>
        {:else if userIsHost}
            <div class="badge preset-filled-primary-500 px-3 py-1 text-xs font-black uppercase">
                Leader
            </div>
        {:else if isMe}
            <div class="badge preset-filled-success-500 px-3 py-1 text-xs font-black uppercase">
                You
            </div>
        {/if}
    </div>
</div>
