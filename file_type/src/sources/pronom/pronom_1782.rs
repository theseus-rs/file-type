use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1782: FileType = FileType {
    file_format: &FileFormat {
        id: 1_782,
        source_type: SourceType::Pronom,
        name: "AutoCAD Design Web Format(DWFx)",
        extensions: &["dwfx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_456,
        }],
    },
};
