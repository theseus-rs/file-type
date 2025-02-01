use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_381: FileFormat = FileFormat {
    id: 559,
    puid: "x-fmt/381",
    name: "Dia Graphics Format",
    extensions: &["dia"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
