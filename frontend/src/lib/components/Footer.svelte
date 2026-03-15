<script>
    import { SiDiscord, SiGithub } from '@icons-pack/svelte-simple-icons';
    import { Lock } from 'lucide-svelte';
</script>

<footer class="border-surface-800 text-surface-500 flex flex-col gap-2 border-t py-4 text-center">
    <p class="text-sm sm:text-sm">
        © 2026 Warframe Relic Finder. Not affiliated with Digital Extremes.
    </p>
    <div class="flex flex-wrap justify-center gap-6">
        <a
            href="https://discord.gg/VANqqkv8pp"
            target="_blank"
            class="text-surface-200 flex items-center gap-1.5 text-sm"
        >
            <SiDiscord /> Discord
        </a>
        <a
            href="https://github.com/Mettwasser/wrf"
            target="_blank"
            class="text-surface-200 flex items-center gap-1.5 text-sm"
        >
            <SiGithub /> GitHub
        </a>
        <a href="/privacy" class="text-surface-200 flex items-center gap-1.5 text-sm">
            <Lock /> Privacy Policy
        </a>
    </div>
</footer>
