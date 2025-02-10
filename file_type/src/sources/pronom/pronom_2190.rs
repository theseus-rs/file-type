use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2190: FileType = FileType {
    file_format: &FileFormat {
        id: 2_190,
        source_type: SourceType::Pronom,
        name: "OmniPage Document",
        extensions: &["opd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_189,
        }],
    },
};
