<script lang="ts">
    import { dark } from '@clerk/themes';
    import { SignedIn, SignedOut, SignIn, SignOutButton } from 'svelte-clerk';
</script>

<SignedOut>
    <SignIn
        appearance={{ captcha: { theme: 'dark' }, theme: dark }}
        redirectUrl="/app/next_steps"
    />
</SignedOut>
<SignedIn>
    <div class="flex flex-col items-center gap-4">
        <h1 class="h2">You are already logged in!</h1>
        <SignOutButton class="btn w-2/3 preset-filled-error-500" />
    </div>
</SignedIn>
