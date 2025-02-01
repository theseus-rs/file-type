use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_165: FileFormat = FileFormat {
    id: 237,
    puid: "x-fmt/165",
    name: "Kodak PhotoCD Image",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
