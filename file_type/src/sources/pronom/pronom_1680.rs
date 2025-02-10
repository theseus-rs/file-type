use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1680: FileType = FileType {
    file_format: &FileFormat {
        id: 1_680,
        source_type: SourceType::Pronom,
        name: "Pagemaker Document (Generic)",
        extensions: &["p65", "pmd", "pmt"],
        media_types: &["application/vnd.pagemaker"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 247,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 254,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_554,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_555,
            },
        ],
    },
};
