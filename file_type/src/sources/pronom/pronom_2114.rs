use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2114: FileType = FileType {
    file_format: &FileFormat {
        id: 2_114,
        source_type: SourceType::Pronom,
        name: "Calendar Creator File",
        extensions: &["cc3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_115,
        }],
    },
};
