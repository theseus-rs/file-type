use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1311: FileType = FileType {
    file_format: &FileFormat {
        id: 1_311,
        source_type: SourceType::Pronom,
        name: "Microsoft Office Theme",
        extensions: &["thmx"],
        media_types: &["application/vnd.ms-officetheme"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 941,
        }],
    },
};
