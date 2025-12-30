use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_3938: FileType = FileType {
    file_format: &FileFormat {
        id: 3_938,
        source_type: SourceType::Pronom,
        name: "DaVinci Resolve Timeline File",
        extensions: &["drt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 3_939,
        }],
    },
};
