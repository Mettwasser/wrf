<script lang="ts">
    import type { Snippet, SvelteComponent } from 'svelte';

    type Props<
        ComponentProps extends Record<string, any> = Record<string, any>,
        Events extends Record<string, any> = any,
        Slots extends Record<string, any> = any,
    > = {
        title: string;
    } & (
        | {
              value: string | undefined;
              Icon?: typeof SvelteComponent<ComponentProps, Events, Slots>;
              children?: never;
          }
        | {
              children: Snippet;
              value?: never;
              Icon?: never;
          }
    );

    let { title, Icon, value, children }: Props = $props();
</script>

<div class="flex w-1/3 flex-col">
    <h1 class="h1">{title}</h1>
    {#if children}
        {@render children()}
    {:else}
        <div class="input-group hidden w-full grid-cols-[auto_1fr] hover:preset-tonal xsm:flex">
            {#if Icon}
                <div class="input-group-cell">
                    <Icon />
                </div>
            {/if}
            <input type="text" class="input text-center" {value} disabled />
        </div>
    {/if}
</div>
