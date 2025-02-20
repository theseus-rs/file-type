use crate::FileType;
use crate::format::{FileFormat, RelatedFormat, RelationshipType, SourceType};

pub(crate) const PRONOM_1523: FileType = FileType {
    file_format: &FileFormat {
        id: 1_523,
        source_type: SourceType::Pronom,
        name: "Keyhole Markup Language (Container)",
        extensions: &["kmz"],
        media_types: &["application/vnd.google-earth.kmz"],
        signatures: &[],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::EquivalentTo,
            id: 975,
        }],
    },
};
