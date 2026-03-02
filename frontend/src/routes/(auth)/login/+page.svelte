<script lang="ts">
    import { goto } from '$app/navigation';
    import { dark } from '@clerk/themes';
    import { SignedIn, SignedOut, SignIn, SignOutButton, useClerkContext } from 'svelte-clerk';

    const clerkCtx = useClerkContext();

    $effect(() => {
        if (clerkCtx.session) goto('/app/next_steps');
    });
</script>

<SignedOut>
    <SignIn appearance={{ captcha: { theme: 'dark' }, theme: dark }} routing="virtual" />
</SignedOut>
<SignedIn>
    <div class="flex flex-col items-center gap-4">
        <h1 class="h2">You are already logged in!</h1>
        <SignOutButton class="btn preset-filled-error-500 w-2/3" />
    </div>
</SignedIn>
