use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_515: FileType = FileType {
    file_format: &FileFormat {
        id: 515,
        source_type: SourceType::Pronom,
        name: "PageMaker Document",
        extensions: &["pm3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 516,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_522,
            },
        ],
    },
};
