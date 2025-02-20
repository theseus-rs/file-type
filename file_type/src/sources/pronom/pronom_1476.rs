use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1476: FileType = FileType {
    file_format: &FileFormat {
        id: 1_476,
        source_type: SourceType::Pronom,
        name: "Serif PagePlus Publication",
        extensions: &["ppp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_470,
        }],
    },
};
