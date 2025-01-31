use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_111: FileFormat = FileFormat {
    id: 163,
    puid: "x-fmt/111",
    name: "Plain Text File",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
