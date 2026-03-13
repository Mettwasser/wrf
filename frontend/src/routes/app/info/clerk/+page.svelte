<script lang="ts">
    import { ArrowLeft } from 'lucide-svelte';
    import { UserProfile } from 'svelte-clerk';
</script>

<div class="flex min-h-full flex-1 flex-col items-center justify-center gap-4 p-4">
    <div class="flex flex-col items-center gap-4 md:items-start">
        <a href="/app/info" class="btn preset-filled-surface-900-100 flex items-center gap-2">
            <ArrowLeft />
            <span>Back to Account Info</span>
        </a>
        <UserProfile />
    </div>
</div>
