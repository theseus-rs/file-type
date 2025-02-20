use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2627: FileType = FileType {
    file_format: &FileFormat {
        id: 2_627,
        source_type: SourceType::Pronom,
        name: "SIARD (Software-Independent Archiving of Relational Databases)",
        extensions: &["siard"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_006,
        }],
    },
};
