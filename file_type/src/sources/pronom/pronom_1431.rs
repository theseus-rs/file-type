use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1431: FileType = FileType {
    file_format: &FileFormat {
        id: 1_431,
        source_type: SourceType::Pronom,
        name: "Microsoft PowerPoint Macro-Enabled Template",
        extensions: &["potm"],
        media_types: &["application/vnd.ms-powerpoint.template.macroEnabled.12"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 941,
        }],
    },
};
