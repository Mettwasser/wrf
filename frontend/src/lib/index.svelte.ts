// place files you want to import through the `$lib` alias in this folder.

export enum NavbarDisplayOptions {
    Hidden,
    ThemesOnly,
    Full,
}

export const navbarOpts = $state({ display: NavbarDisplayOptions.Full });
