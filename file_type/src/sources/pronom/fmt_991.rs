use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_991: FileFormat = FileFormat {
    id: 1_796,
    puid: "fmt/991",
    name: "SHA256 File",
    extensions: &["sha256"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
