use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2427: FileType = FileType {
    file_format: &FileFormat {
        id: 2_427,
        source_type: SourceType::Pronom,
        name: "ESRI ArcInfo DAT File (Internal)",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::EquivalentTo,
            id: 2_421,
        }],
    },
};
