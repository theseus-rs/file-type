use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_96: FileFormat = FileFormat {
    id: 144,
    puid: "x-fmt/96",
    name: "Pocket Word Template",
    extensions: &["pwt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
