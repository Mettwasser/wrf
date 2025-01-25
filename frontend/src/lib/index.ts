// place files you want to import through the `$lib` alias in this folder.

import { dev } from '$app/environment';
import { PersistedState } from 'runed';
import type { User } from './types/user';
import type { Theme } from '@skeletonlabs/skeleton/themes';
import * as themes from '@skeletonlabs/skeleton/themes';
import type { DarkModeState } from './utils/darkMode';
import type { Snippet } from 'svelte';
import * as combobox from '@zag-js/combobox';

export enum NavbarDisplayOptions {
    Hidden,
    ThemesOnly,
    Full,
}

export const URL = dev ? 'http://localhost:5150' : 'TODO';

export const theme = new PersistedState<Theme>('theme', themes.cerberus);
export const darkModeState = new PersistedState<DarkModeState>('darkModeState', 'dark');

export const dateReviver = (k: any, v: any) => {
    if (typeof v === 'string') {
        let d = new Date(v);
        if (Number.isNaN(d.getTime())) return v;
        return d;
    }
    return v;
};
export const useId = (() => {
    let id = 0;
    return () => Math.random().toString(36).substring(2) + id++;
})();

export interface ComboboxProps
    extends Omit<combobox.Context, 'id' | 'collection' | 'value' | 'label' | 'multiple'> {
    /** Provide the list of label and value data */
    data?: { label: string; value: string }[];
    /** Bind the selected value. */
    value?: string[] | undefined;
    /** Set the label to display. */
    label?: string;

    onInputChange?: (input: string) => void;

    limit?: number;
    searchThreshold?: number;

    // Base ---
    /** Set base classes for the root element. */
    base?: string;
    /** Set width classes for the root element. */
    width?: string;
    /** Provide arbitrary classes for the root element. */
    classes?: string;

    // Label ---
    /** Set base classes for the label. */
    labelBase?: string;
    /** Set text and font classes for the label. */
    labelText?: string;
    /** Provide arbitrary classes for the label. */
    labelClasses?: string;

    // Input Group ---
    /** Set base classes for the input group. */
    inputGroupBase?: string;
    /** Set input classes for the input group. */
    inputGroupInput?: string;
    /** Set button classes for the input group. */
    inputGroupButton?: string;
    /** Set arrow classes for the input group. */
    inputGroupArrow?: string;
    /** Provide arbitrary classes for the input group. */
    inputGroupClasses?: string;

    // Positioner ---
    /** Set base classes for the positioner. */
    positionerBase?: string;
    /** Set z-index classes for the positioner. */
    positionerZIndex?: string;
    /** Provide arbitrary classes for the positioner. */
    positionerClasses?: string;

    // Content ---
    /** Set base classes for the content. */
    contentBase?: string;
    /** Set background classes for the content. */
    contentBackground?: string;
    /** Set space-y classes for the content. */
    contentSpaceY?: string;
    /** Provide arbitrary classes for the content. */
    contentClasses?: string;

    // Option ---
    /** Set base classes for the option. */
    optionBase?: string;
    /** Set focus classes for the option. */
    optionFocus?: string;
    /** Set hover classes for the option. */
    optionHover?: string;
    /** Set active classes for the option. */
    optionActive?: string;
    /** Provide arbitrary classes for the option. */
    optionClasses?: string;

    // Snippets ---
    /** Provide a custom arrow icon. */
    arrow?: Snippet;

    // Events ---
    /** Handle the combobox dropdown button click event. */
    onclick?: (event: Event) => void;
}
