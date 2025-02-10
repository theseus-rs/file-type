use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2795: FileType = FileType {
    file_format: &FileFormat {
        id: 2_795,
        source_type: SourceType::Pronom,
        name: "CorelDraw Drawing",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_794,
        }],
    },
};
