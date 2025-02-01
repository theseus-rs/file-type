use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_145: FileFormat = FileFormat {
    id: 206,
    puid: "x-fmt/145",
    name: "Stats+ Data File",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
