use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_11: FileType = FileType {
    file_format: &FileFormat {
        id: 11,
        source_type: SourceType::Pronom,
        name: "Microsoft Word for Macintosh Document",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_091,
        }],
    },
};
