use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_99: FileFormat = FileFormat {
    id: 147,
    puid: "x-fmt/99",
    name: "Schedule+ Contacts",
    extensions: &["scd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
