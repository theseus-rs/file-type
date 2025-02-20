use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1429: FileType = FileType {
    file_format: &FileFormat {
        id: 1_429,
        source_type: SourceType::Pronom,
        name: "Microsoft PowerPoint Macro-Enabled Show",
        extensions: &["ppsm"],
        media_types: &["application/vnd.ms-powerpoint.slideshow.macroEnabled.12"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 941,
        }],
    },
};
