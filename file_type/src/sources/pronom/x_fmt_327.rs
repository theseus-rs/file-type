use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_327: FileFormat = FileFormat {
    id: 490,
    puid: "x-fmt/327",
    name: "IntelliDraw Vector Graphics",
    extensions: &["idw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
