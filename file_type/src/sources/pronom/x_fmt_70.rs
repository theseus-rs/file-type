use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_70: FileFormat = FileFormat {
    id: 111,
    puid: "x-fmt/70",
    name: "AutoCAD Menu Resource File",
    extensions: &["mnr", "mnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
