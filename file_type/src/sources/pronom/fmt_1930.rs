use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1930: FileFormat = FileFormat {
    id: 2_791,
    puid: "fmt/1930",
    name: "CorelDraw Drawing",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_792,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_790,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
