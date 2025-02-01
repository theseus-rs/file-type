use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_358: FileFormat = FileFormat {
    id: 524,
    puid: "x-fmt/358",
    name: "Silicon Graphics Graphics File",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
