use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1644: FileFormat = FileFormat {
    id: 2_471,
    puid: "fmt/1644",
    name: "Roxio Label Creator Project File",
    extensions: &["jwl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_472,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
