use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1416: FileType = FileType {
    file_format: &FileFormat {
        id: 1_416,
        source_type: SourceType::Pronom,
        name: "GeoGebra",
        extensions: &["ggb"],
        media_types: &["application/vnd.geogebra.file"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_417,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_413,
            },
        ],
    },
};
