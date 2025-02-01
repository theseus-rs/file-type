use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_286: FileFormat = FileFormat {
    id: 437,
    puid: "x-fmt/286",
    name: "DEC Data Exchange File",
    extensions: &["dx"],
    media_types: &["application/dec-dx."],
    internal_signatures: &[],
    related_formats: &[],
};
