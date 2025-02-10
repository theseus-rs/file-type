use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2352: FileType = FileType {
    file_format: &FileFormat {
        id: 2_352,
        source_type: SourceType::Pronom,
        name: "Serif DrawPlus Drawing",
        extensions: &["dpp", "dpa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_628,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_653,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_654,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_343,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_344,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_345,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_346,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_347,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_348,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_349,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_350,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_351,
            },
        ],
    },
};
