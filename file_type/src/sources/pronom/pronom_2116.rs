use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2116: FileType = FileType {
    file_format: &FileFormat {
        id: 2_116,
        source_type: SourceType::Pronom,
        name: "Calendar Creator File",
        extensions: &["bcc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_115,
        }],
    },
};
