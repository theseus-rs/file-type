use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2165: FileType = FileType {
    file_format: &FileFormat {
        id: 2_165,
        source_type: SourceType::Pronom,
        name: "Autodesk Revit Project File",
        extensions: &["rvt", "rte", "rft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_166,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_167,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 2_164,
            },
        ],
    },
};
