use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2396: FileType = FileType {
    file_format: &FileFormat {
        id: 2_396,
        source_type: SourceType::Pronom,
        name: "ISDOCX Information System Document",
        extensions: &["isdocx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 382,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 2_393,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 2_395,
            },
        ],
    },
};
