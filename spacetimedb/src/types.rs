use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Debug, Clone, PartialEq, Eq)]
pub enum Region {
    AS,
    EER,
    EU,
    NA,
    OC,
    SA,
}

#[derive(SpacetimeType, Debug, Clone, PartialEq, Eq)]
pub enum RelicRefinement {
    Intact,
    Exceptional,
    Flawless,
    Radiant,
}

#[derive(SpacetimeType, Debug, Clone, PartialEq, Eq)]
pub enum RotationType {
    TwoATwoB,
    FourA,
}
