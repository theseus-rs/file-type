use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1934: FileFormat = FileFormat {
    id: 2_795,
    puid: "fmt/1934",
    name: "CorelDraw Drawing",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_794,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
