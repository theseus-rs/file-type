use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2160: FileType = FileType {
    file_format: &FileFormat {
        id: 2_160,
        source_type: SourceType::Pronom,
        name: "BDOC",
        extensions: &["bdoc", "asice"],
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
