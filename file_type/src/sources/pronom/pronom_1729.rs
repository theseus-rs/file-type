use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1729: FileType = FileType {
    file_format: &FileFormat {
        id: 1_729,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Drawing",
        extensions: &["vsdx"],
        media_types: &["application/vnd.ms-visio.drawing.main+xml"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_230,
        }],
    },
};
