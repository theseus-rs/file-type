use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2006: FileType = FileType {
    file_format: &FileFormat {
        id: 2_006,
        source_type: SourceType::Pronom,
        name: "SIARD (Software-Independent Archiving of Relational Databases)",
        extensions: &["siard"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_627,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_800,
            },
        ],
    },
};
