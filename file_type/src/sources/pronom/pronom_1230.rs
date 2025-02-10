use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1230: FileType = FileType {
    file_format: &FileFormat {
        id: 1_230,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Drawing",
        extensions: &["vsd"],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 767,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_229,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_729,
            },
        ],
    },
};
