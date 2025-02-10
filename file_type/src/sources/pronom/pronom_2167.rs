use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2167: FileType = FileType {
    file_format: &FileFormat {
        id: 2_167,
        source_type: SourceType::Pronom,
        name: "Autodesk Revit Family File",
        extensions: &["rfa", "rft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 2_164,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 2_165,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 2_166,
            },
        ],
    },
};
