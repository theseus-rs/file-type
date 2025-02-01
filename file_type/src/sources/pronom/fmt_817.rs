use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_817: FileFormat = FileFormat {
    id: 1_617,
    puid: "fmt/817",
    name: "JSON Data Interchange Format",
    extensions: &["json"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_129,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
