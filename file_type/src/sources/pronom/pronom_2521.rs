use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2521: FileType = FileType {
    file_format: &FileFormat {
        id: 2_521,
        source_type: SourceType::Pronom,
        name: "EndNote Compressed Library",
        extensions: &["enlx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 382,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_518,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_519,
            },
        ],
    },
};
