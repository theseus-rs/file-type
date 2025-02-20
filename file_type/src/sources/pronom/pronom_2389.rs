use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2389: FileType = FileType {
    file_format: &FileFormat {
        id: 2_389,
        source_type: SourceType::Pronom,
        name: "Associated Signature Container Extended (ASiC-E)",
        extensions: &["asice", "sce"],
        media_types: &["application/vnd.etsi.asic-e+zip"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_069,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_160,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 2_069,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 2_160,
            },
        ],
    },
};
