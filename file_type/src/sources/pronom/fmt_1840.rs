use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1840: FileFormat = FileFormat {
    id: 2_692,
    puid: "fmt/1840",
    name: "WACZ",
    extensions: &["wacz"],
    media_types: &["application/x-wacz"],
    internal_signatures: &[],
    related_formats: &[],
};
