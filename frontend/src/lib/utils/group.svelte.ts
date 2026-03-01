export type Group<T> = {
    value: T;
    isEditing: boolean;
    isSaving: boolean;
    errorText: string | undefined;
    toggleEditing: () => void;
    toggleSaving: () => void;
    withSaving: (callback: () => Promise<void>) => Promise<void>;
};

export function group<T>(initialValue: () => T): Group<T> {
    let state = $state({
        value: initialValue(),
        isEditing: false,
        isSaving: false,
        errorText: undefined as string | undefined,
    });

    $effect(() => {
        const value = initialValue();
        if (!state.isEditing && !state.isSaving) {
            state.value = value;
        }
    });

    return {
        // Getter and Setter for 'value' allows bind:value={myGroup.value}
        get value() {
            return state.value;
        },
        set value(v: T) {
            state.value = v;
        },

        get isEditing() {
            return state.isEditing;
        },
        set isEditing(v: boolean) {
            state.isEditing = v;
        },

        get isSaving() {
            return state.isSaving;
        },
        set isSaving(v: boolean) {
            state.isSaving = v;
        },

        get errorText() {
            return state.errorText;
        },
        set errorText(v: string | undefined) {
            state.errorText = v;
        },

        toggleEditing: () => {
            state.isEditing = !state.isEditing;
        },

        toggleSaving: () => {
            state.isSaving = !state.isSaving;
        },

        withSaving: async (callback: () => Promise<void>) => {
            state.isSaving = true;
            await callback();
            state.isSaving = false;
        },
    };
}
