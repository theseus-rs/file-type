use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1295: FileFormat = FileFormat {
    id: 2_113,
    puid: "fmt/1295",
    name: "Calendar Creator Event",
    extensions: &["ce3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 202,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
