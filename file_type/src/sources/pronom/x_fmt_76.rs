use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_76: FileFormat = FileFormat {
    id: 118,
    puid: "x-fmt/76",
    name: "CorelDraw Pattern",
    extensions: &["pat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
