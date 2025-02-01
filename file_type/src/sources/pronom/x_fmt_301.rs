use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_301: FileFormat = FileFormat {
    id: 455,
    puid: "x-fmt/301",
    name: "ACBM Graphics",
    extensions: &["acb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
