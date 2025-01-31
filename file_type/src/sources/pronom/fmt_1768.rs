use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1768: FileFormat = FileFormat {
    id: 2_618,
    puid: "fmt/1768",
    name: "C Source Code File",
    extensions: &["c"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
