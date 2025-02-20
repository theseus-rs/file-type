use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1391: FileType = FileType {
    file_format: &FileFormat {
        id: 1_391,
        source_type: SourceType::Pronom,
        name: "Microsoft Word Macro-Enabled Document Template",
        extensions: &["dotm"],
        media_types: &["application/vnd.ms-word.template.macroEnabled.12"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 1_160,
        }],
    },
};
