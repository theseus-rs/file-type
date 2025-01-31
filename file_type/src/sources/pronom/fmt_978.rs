use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_978: FileFormat = FileFormat {
    id: 1_783,
    puid: "fmt/978",
    name: "3DS Max",
    extensions: &["max", "chr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_292,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
