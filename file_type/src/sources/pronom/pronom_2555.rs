use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2555: FileType = FileType {
    file_format: &FileFormat {
        id: 2_555,
        source_type: SourceType::Pronom,
        name: "PageMaker Mac Document",
        extensions: &["pm6", "pt6"],
        media_types: &["application/vnd.pagemaker"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::EquivalentTo,
                id: 247,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_680,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_554,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_523,
            },
        ],
    },
};
