use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1084: FileFormat = FileFormat {
    id: 1_892,
    puid: "fmt/1084",
    name: "Hangul Word Processor Document",
    extensions: &["hwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
