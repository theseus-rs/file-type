use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_354: FileFormat = FileFormat {
    id: 520,
    puid: "x-fmt/354",
    name: "SAP Document",
    extensions: &["ali"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
