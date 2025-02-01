use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_52: FileFormat = FileFormat {
    id: 85,
    puid: "x-fmt/52",
    name: "Drawing Interchange Format Style Extract",
    extensions: &["dxx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
