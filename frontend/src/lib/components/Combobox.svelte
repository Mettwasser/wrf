<script lang="ts">
    import {
        Combobox,
        Portal,
        type ComboboxRootProps,
        useListCollection,
    } from '@skeletonlabs/skeleton-svelte';

    interface Props {
        data: { label: string; value: string }[];
        value: string[] | undefined;
        placeholder: string;
        limit?: number;
    }

    let { data, value = $bindable(), placeholder, limit }: Props = $props();

    let items = $state(data);
    const limitedItems = $derived(limit ? items.slice(0, limit) : items);

    let inputValue: string | undefined = $state(undefined);

    const collection = $derived(
        useListCollection({
            items: limitedItems,
            itemToString: (item) => item.label,
            itemToValue: (item) => item.value,
        })
    );

    const onOpenChange: ComboboxRootProps['onOpenChange'] = (details) => {
        console.log(details);
        items = data;
        if (!details.open) {
            console.log('value: ', value);
            inputValue = value?.[0];
            console.log('Input value after change: ', inputValue);
        }
    };

    const onValueChange: ComboboxRootProps['onValueChange'] = (details) => {
        value = details.value;
        inputValue = details.value[0];
    };

    const onInputValueChange: ComboboxRootProps['onInputValueChange'] = (event) => {
        const filtered = data.filter((item) =>
            item.value.toLowerCase().includes(event.inputValue.toLowerCase())
        );
        if (filtered.length > 0) {
            items = filtered;
        } else {
            items = [];
        }
    };
</script>

<Combobox
    class="max-w-md"
    {placeholder}
    {collection}
    {onOpenChange}
    {onInputValueChange}
    {onValueChange}
    {value}
    {inputValue}
>
    <Combobox.Control>
        <Combobox.Input />
        <Combobox.Trigger />
    </Combobox.Control>
    <Portal>
        <Combobox.Positioner>
            <Combobox.Content class="z-50">
                {#each limitedItems as item (item.value)}
                    <Combobox.Item {item}>
                        <Combobox.ItemText>{item.label}</Combobox.ItemText>
                        <Combobox.ItemIndicator />
                    </Combobox.Item>
                {/each}
            </Combobox.Content>
        </Combobox.Positioner>
    </Portal>
</Combobox>
