use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_859: FileType = FileType {
    file_format: &FileFormat {
        id: 859,
        source_type: SourceType::Pronom,
        name: "Revit Template",
        extensions: &["rte"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 767,
        }],
    },
};
