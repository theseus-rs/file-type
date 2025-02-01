use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_57: FileFormat = FileFormat {
    id: 91,
    puid: "x-fmt/57",
    name: "Ventura Publisher Vector Graphics",
    extensions: &["gem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
