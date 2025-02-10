use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2354: FileType = FileType {
    file_format: &FileFormat {
        id: 2_354,
        source_type: SourceType::Pronom,
        name: "Serif PagePlus Publication",
        extensions: &["ppp", "ppb", "ppx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_470,
        }],
    },
};
