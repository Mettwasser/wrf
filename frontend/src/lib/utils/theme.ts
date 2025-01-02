import * as themes from '@skeletonlabs/skeleton/themes';
import { getWithFallback } from './localstorage';

type Theme = typeof themes.vintage;

export const THEMES: Theme[] = Object.values(themes);

export function updateThemeInHtml(theme: Theme) {
    const body = document.querySelector('body')!;

    body.dataset.theme = theme.name;
}
