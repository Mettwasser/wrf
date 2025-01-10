<script lang="ts">
    import { page } from '$app/state';
</script>

<div class="flex flex-1 flex-col items-center justify-center">
    <h1 class="text-9xl">{page.status}</h1>
    <h2 class="h2">{page.error?.message}</h2>
</div>
