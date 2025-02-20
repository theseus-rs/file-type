use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1427: FileType = FileType {
    file_format: &FileFormat {
        id: 1_427,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel Macro-Enabled Add-In",
        extensions: &["xlam"],
        media_types: &["application/vnd.ms-excel.addin.macroEnabled.12"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 940,
        }],
    },
};
