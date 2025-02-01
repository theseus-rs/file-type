use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1187: FileFormat = FileFormat {
    id: 1_997,
    puid: "fmt/1187",
    name: "Apple iWork Template",
    extensions: &["template"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
