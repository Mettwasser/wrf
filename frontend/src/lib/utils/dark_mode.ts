export type DarkModeState = 'light' | 'dark';

export function updateDarkModeInHtml(state: DarkModeState) {
    const html = document.querySelector('html')!;
    if (state === 'dark') html.classList.add('dark');
    else if (state === 'light') html.classList.remove('dark');
}
