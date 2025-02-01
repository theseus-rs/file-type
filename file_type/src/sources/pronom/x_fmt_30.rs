use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_30: FileFormat = FileFormat {
    id: 61,
    puid: "x-fmt/30",
    name: "CorelDraw Template",
    extensions: &["cdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
