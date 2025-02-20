use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2401: FileType = FileType {
    file_format: &FileFormat {
        id: 2_401,
        source_type: SourceType::Pronom,
        name: "Spectrum 512 Uncompressed | Spectrum 512 Uncompressed Enhanced",
        extensions: &["spu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 687,
        }],
    },
};
