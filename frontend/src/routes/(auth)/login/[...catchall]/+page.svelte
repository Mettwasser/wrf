<script lang="ts">
    import { dark } from '@clerk/themes';
    import { Show, SignIn, SignOutButton } from 'svelte-clerk';
</script>

<Show when="signed-out">
    <SignIn forceRedirectUrl="/app/next_steps" appearance={{ theme: dark }} />
</Show>
<Show when="signed-in">
    <div class="flex flex-col items-center justify-center gap-4">
        <h1 class="h2 text-center">You are already logged in!</h1>
        <SignOutButton class="btn preset-filled-error-500 w-2/3" />
    </div>
</Show>
