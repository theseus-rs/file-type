use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2189: FileType = FileType {
    file_format: &FileFormat {
        id: 2_189,
        source_type: SourceType::Pronom,
        name: "OmniPage Pro Document",
        extensions: &["opd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_190,
        }],
    },
};
