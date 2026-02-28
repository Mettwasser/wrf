export const THEMES = [
    'mona',
    'vox',
    'catppuccin',
    'crimson',
    'legacy',
    'pine',
    'terminus',
    'wintry',
    'cerberus',
    'fennec',
    'mint',
    'nosh',
    'sahara',
    'concord',
    'hamlindigo',
    'modern',
    'nouveau',
    'rocket',
    'seafoam',
];

export function updateThemeInHtml(theme: string) {
    const body = document.querySelector('html')!;
    body.dataset.theme = theme;
}
