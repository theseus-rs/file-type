use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_361: FileFormat = FileFormat {
    id: 527,
    puid: "x-fmt/361",
    name: "StatGraphics Data File",
    extensions: &["aws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
