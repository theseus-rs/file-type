use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_349: FileFormat = FileFormat {
    id: 513,
    puid: "x-fmt/349",
    name: "Nota Bene Text File",
    extensions: &["nb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
