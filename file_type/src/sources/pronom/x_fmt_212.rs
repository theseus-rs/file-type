use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_212: FileFormat = FileFormat {
    id: 299,
    puid: "x-fmt/212",
    name: "Lotus 1-2-3 Worksheet",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
