use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_938: FileFormat = FileFormat {
    id: 1_743,
    puid: "fmt/938",
    name: "Python Source Code File",
    extensions: &["py"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
