use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_282: FileFormat = FileFormat {
    id: 433,
    puid: "x-fmt/282",
    name: "8-bit ANSI Text",
    extensions: &["ans"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
