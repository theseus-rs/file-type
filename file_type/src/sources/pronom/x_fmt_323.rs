use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_323: FileFormat = FileFormat {
    id: 486,
    puid: "x-fmt/323",
    name: "Framework Database",
    extensions: &["fw4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
