use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2360: FileType = FileType {
    file_format: &FileFormat {
        id: 2_360,
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
