use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_110: FileFormat = FileFormat {
    id: 162,
    puid: "x-fmt/110",
    name: "Fixed Width Values Text File",
    extensions: &[],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
