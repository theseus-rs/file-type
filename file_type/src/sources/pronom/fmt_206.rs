use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_206: FileFormat = FileFormat {
    id: 932,
    puid: "fmt/206",
    name: "Structured Query Language Data",
    extensions: &["sql"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
