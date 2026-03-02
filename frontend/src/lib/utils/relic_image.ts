import type { RelicRefinement } from '$lib/module_bindings/types';

type RelicMap = Record<RelicRefinement['tag'], string>;

const LITH_MAP: RelicMap = {
    Intact: 'https://wiki.warframe.com/images/LithRelicIntact.png?ee7d7',
    Exceptional: 'https://wiki.warframe.com/images/LithRelicExceptional.png?9e162',
    Flawless: 'https://wiki.warframe.com/images/LithRelicFlawless.png?9e4fa',
    Radiant: 'https://wiki.warframe.com/images/LithRelicRadiant.png?e59d0',
};
const MESO_MAP: RelicMap = {
    Intact: 'https://wiki.warframe.com/images/MesoRelicIntact.png?a9b4a',
    Exceptional: 'https://wiki.warframe.com/images/MesoRelicExceptional.png?a928b',
    Flawless: 'https://wiki.warframe.com/images/MesoRelicFlawless.png?e337c',
    Radiant: 'https://wiki.warframe.com/images/MesoRelicRadiant.png?e337c',
};
const NEO_MAP: RelicMap = {
    Intact: 'https://wiki.warframe.com/images/NeoRelicIntact.png?6dc86',
    Exceptional: 'https://wiki.warframe.com/images/NeoRelicExceptional.png?8c0d7',
    Flawless: 'https://wiki.warframe.com/images/NeoRelicFlawless.png?44f59',
    Radiant: 'https://wiki.warframe.com/images/NeoRelicRadiant.png?5e94b',
};
const AXI_MAP: RelicMap = {
    Intact: 'https://wiki.warframe.com/images/AxiRelicIntact.png?6cadf',
    Exceptional: 'https://wiki.warframe.com/images/AxiRelicExceptional.png?7edaa',
    Flawless: 'https://wiki.warframe.com/images/AxiRelicFlawless.png?54d33',
    Radiant: 'https://wiki.warframe.com/images/AxiRelicRadiant.png?cf015',
};

export function getRelicImageUrl(activity: string, refinement: RelicRefinement) {
    switch (activity.split(' ')[0]!) {
        case 'Lith':
            return LITH_MAP[refinement.tag];
        case 'Meso':
            return MESO_MAP[refinement.tag];
        case 'Neo':
            return NEO_MAP[refinement.tag];
        case 'Axi':
            return AXI_MAP[refinement.tag];

        default:
            return '';
    }
}
