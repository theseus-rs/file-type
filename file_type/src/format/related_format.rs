/// The type of relationship between two format
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum RelationshipType {
    CanContain,
    CanBeContainedBy,
    #[default]
    EquivalentTo,
    HasPriorityOver,
    HasLowerPriorityThan,
    IsPreviousVersionOf,
    IsSubsequentVersionOf,
    IsSubtypeOf,
    IsSupertypeOf,
}

/// A related format
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RelatedFormat {
    pub relationship_type: RelationshipType,
    pub id: usize,
}
