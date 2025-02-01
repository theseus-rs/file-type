use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1298: FileFormat = FileFormat {
    id: 2_116,
    puid: "fmt/1298",
    name: "Calendar Creator File",
    extensions: &["bcc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_115,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
