use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_289: FileFormat = FileFormat {
    id: 440,
    puid: "x-fmt/289",
    name: "IBM DisplayWrite Document",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
