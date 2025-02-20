use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1654: FileType = FileType {
    file_format: &FileFormat {
        id: 1_654,
        source_type: SourceType::Pronom,
        name: "Serif DrawPlus Drawing",
        extensions: &["dpp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 2_352,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_653,
            },
        ],
    },
};
