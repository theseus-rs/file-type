use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_362: FileFormat = FileFormat {
    id: 528,
    puid: "x-fmt/362",
    name: "StratGraphics Data File",
    extensions: &["asf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
