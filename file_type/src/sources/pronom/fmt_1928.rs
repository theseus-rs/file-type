use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1928: FileFormat = FileFormat {
    id: 2_789,
    puid: "fmt/1928",
    name: "CorelDraw Drawing",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_790,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
