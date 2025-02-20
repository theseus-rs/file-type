use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_861: FileType = FileType {
    file_format: &FileFormat {
        id: 861,
        source_type: SourceType::Pronom,
        name: "Revit External Group",
        extensions: &["rvg"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 767,
        }],
    },
};
