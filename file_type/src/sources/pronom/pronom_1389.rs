use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1389: FileType = FileType {
    file_format: &FileFormat {
        id: 1_389,
        source_type: SourceType::Pronom,
        name: "Microsoft Word Template",
        extensions: &["dotx"],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.template"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 1_160,
        }],
    },
};
