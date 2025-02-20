use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_858: FileType = FileType {
    file_format: &FileFormat {
        id: 858,
        source_type: SourceType::Pronom,
        name: "Revit Family Template",
        extensions: &["rft"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 767,
        }],
    },
};
