use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_11: FileFormat = FileFormat {
    id: 27,
    puid: "x-fmt/11",
    name: "Revisable-Form-Text Document Content Architecture",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
