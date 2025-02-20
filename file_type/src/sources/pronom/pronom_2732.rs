use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2732: FileType = FileType {
    file_format: &FileFormat {
        id: 2_732,
        source_type: SourceType::Pronom,
        name: "GST Art File",
        extensions: &["art"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_731,
        }],
    },
};
