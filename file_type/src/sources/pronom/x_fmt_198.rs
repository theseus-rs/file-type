use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_198: FileFormat = FileFormat {
    id: 272,
    puid: "x-fmt/198",
    name: "Pagemaker TableEditor Graphics",
    extensions: &["tbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
