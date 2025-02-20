use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_857: FileType = FileType {
    file_format: &FileFormat {
        id: 857,
        source_type: SourceType::Pronom,
        name: "Revit Family File",
        extensions: &["rfa"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 767,
        }],
    },
};
