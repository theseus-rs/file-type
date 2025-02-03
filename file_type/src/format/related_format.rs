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
    pub relationship_type: RelationshipType,
    pub id: usize,
}

impl Source for RelatedFormat {
    fn to_source(&self) -> String {
        format!(
            "RelatedFormat {{ relationship_type: RelationshipType::{:?}, id: {} }}",
            self.relationship_type,
            self.id.to_source(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_source() {
        let related_format = RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1617,
        };
        assert_eq!(
            related_format.to_source(),
            "RelatedFormat { relationship_type: RelationshipType::HasPriorityOver, id: 1_617 }"
        );
    }
}
