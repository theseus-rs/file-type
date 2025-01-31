use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_83: FileFormat = FileFormat {
    id: 127,
    puid: "x-fmt/83",
    name: "Hewlett Packard Vector Graphic Plotter File",
    extensions: &["plt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
