use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_55: FileType = FileType {
    file_format: &FileFormat {
        id: 55,
        source_type: SourceType::Pronom,
        name: "OS/2 Bitmap",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::EquivalentTo,
            id: 728,
        }],
    },
};
