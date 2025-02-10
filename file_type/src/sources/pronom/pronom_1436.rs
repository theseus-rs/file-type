use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1436: FileType = FileType {
    file_format: &FileFormat {
        id: 1_436,
        source_type: SourceType::Pronom,
        name: "Microsoft PowerPoint Macro-Enabled Slide",
        extensions: &["sldm"],
        media_types: &["application/vnd.ms-powerpoint.slide.macroEnabled.12"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 941,
        }],
    },
};
