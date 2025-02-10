use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1430: FileType = FileType {
    file_format: &FileFormat {
        id: 1_430,
        source_type: SourceType::Pronom,
        name: "Microsoft PowerPoint Template",
        extensions: &["potx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.template"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 941,
        }],
    },
};
