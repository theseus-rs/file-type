use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_2169: FileType = FileType {
    file_format: &FileFormat {
        id: 2_169,
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
                id: 2_168,
            },
        ],
    },
};
