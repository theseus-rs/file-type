use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_424: FileFormat = FileFormat {
    id: 811,
    puid: "x-fmt/424",
    name: "Deluxe Paint bitmap",
    extensions: &["lbm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
