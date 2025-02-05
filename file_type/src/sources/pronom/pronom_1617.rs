use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1617: FileFormat = FileFormat {
    id: 1_617,
    source_type: SourceType::Pronom,
    name: "JSON Data Interchange Format",
    extensions: &["json"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasLowerPriorityThan,
        id: 2_129,
    }],
};
