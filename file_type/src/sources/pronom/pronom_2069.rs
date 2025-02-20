use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2069: FileType = FileType {
    file_format: &FileFormat {
        id: 2_069,
        source_type: SourceType::Pronom,
        name: "Electronically Certified Document (EDOC)",
        extensions: &["edoc"],
        media_types: &["application/vnd.etsi.asic-e+zip"],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 2_389,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 2_389,
            },
        ],
    },
};
