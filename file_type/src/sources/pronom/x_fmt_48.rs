use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_48: FileFormat = FileFormat {
    id: 79,
    puid: "x-fmt/48",
    name: "Visual Basic Macro",
    extensions: &["dvb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
