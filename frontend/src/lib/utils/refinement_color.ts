import type { RelicRefinement } from '$lib/module_bindings/types';

type RefinementColorMap = Record<RelicRefinement['tag'], string>;

const REFINEMENT_GRADIENTS: Record<keyof RefinementColorMap, string> = {
    // A subtle copper-to-bronze transition
    Intact: 'bg-linear-135 from-bronze-light to-bronze bg-clip-text text-transparent opacity-90',

    // High-contrast "Chrome" silver
    Exceptional:
        'bg-linear-135 from-silver-light via-silver to-silver-light bg-clip-text text-transparent',

    // The classic "Gold Bar" look
    Flawless:
        'bg-linear-135 from-gold-light via-gold to-gold-light bg-clip-text text-transparent drop-shadow-sm',

    // Deep, saturated "Magic" purple
    Radiant: 'bg-linear-135 from-royal-purple-light to-royal-purple bg-clip-text text-transparent',
};

export function getRefinementTextColor(refinement: RelicRefinement) {
    return REFINEMENT_GRADIENTS[refinement.tag];
}
