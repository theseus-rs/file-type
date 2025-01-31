use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_657: FileFormat = FileFormat {
    id: 1_456,
    puid: "fmt/657",
    name: "Open XML Paper Specification",
    extensions: &["xps", "oxps"],
    media_types: &["application/oxps"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 1_630,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_782,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
