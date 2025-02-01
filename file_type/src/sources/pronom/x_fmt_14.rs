use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_14: FileFormat = FileFormat {
    id: 41,
    puid: "x-fmt/14",
    name: "Macintosh Text File",
    extensions: &[],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
