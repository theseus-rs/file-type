use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_934: FileType = FileType {
    file_format: &FileFormat {
        id: 934,
        source_type: SourceType::Pronom,
        name: "Binary File",
        extensions: &["bin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_439,
        }],
    },
};
