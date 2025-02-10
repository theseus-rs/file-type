use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1390: FileType = FileType {
    file_format: &FileFormat {
        id: 1_390,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel Template",
        extensions: &["xltx"],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.template"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 940,
        }],
    },
};
