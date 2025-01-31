use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_159: FileFormat = FileFormat {
    id: 824,
    puid: "fmt/159",
    name: "EBCDIC-US",
    extensions: &["ebcdic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
