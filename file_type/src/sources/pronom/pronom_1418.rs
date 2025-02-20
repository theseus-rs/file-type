use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1418: FileType = FileType {
    file_format: &FileFormat {
        id: 1_418,
        source_type: SourceType::Pronom,
        name: "GeoGebra",
        extensions: &["ggb"],
        media_types: &["application/vnd.geogebra.file"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_417,
        }],
    },
};
