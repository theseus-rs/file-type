use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_796: FileType = FileType {
    file_format: &FileFormat {
        id: 796,
        source_type: SourceType::Pronom,
        name: "Tagged Image File Format for Image Technology (TIFF/IT)",
        extensions: &["tif", "tiff"],
        media_types: &["image/tiff"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 612,
        }],
    },
};
