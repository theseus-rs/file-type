use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2344: FileType = FileType {
    file_format: &FileFormat {
        id: 2_344,
        source_type: SourceType::Pronom,
        name: "Serif DrawPlus Drawing",
        extensions: &["dpp", "dpa", "dpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_352,
        }],
    },
};
