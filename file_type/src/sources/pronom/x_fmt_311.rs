use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_311: FileFormat = FileFormat {
    id: 469,
    puid: "x-fmt/311",
    name: "dBASE Text Memo",
    extensions: &["dbt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
