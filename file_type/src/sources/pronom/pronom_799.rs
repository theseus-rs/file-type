use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_799: FileType = FileType {
    file_format: &FileFormat {
        id: 799,
        source_type: SourceType::Pronom,
        name: "Tagged Image File Format for Internet Fax (TIFF-FX)",
        extensions: &["tif", "tiff", "tfx"],
        media_types: &["image/tiff"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 672,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 673,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 752,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_099,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 612,
            },
        ],
    },
};
