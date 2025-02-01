use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_213: FileFormat = FileFormat {
    id: 939,
    puid: "fmt/213",
    name: "ScanIt Document",
    extensions: &["sid"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
