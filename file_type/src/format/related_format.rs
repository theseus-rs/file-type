use crate::format::source::Source;

/// The type of relationship between two format
#[derive(Clone, Debug, Default)]
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
#[derive(Clone, Debug, Default)]
pub struct RelatedFormat {
    pub id: usize,
    pub relationship_type: RelationshipType,
}

impl Source for RelatedFormat {
    fn to_source(&self) -> String {
        format!(
            "RelatedFormat {{ id: {}, relationship_type: RelationshipType::{:?} }}",
            self.id.to_source(),
            self.relationship_type,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_source() {
        let related_format = RelatedFormat {
            id: 1617,
            relationship_type: RelationshipType::HasPriorityOver,
        };
        assert_eq!(
            related_format.to_source(),
            "RelatedFormat { id: 1_617, relationship_type: RelationshipType::HasPriorityOver }"
        );
    }
}
