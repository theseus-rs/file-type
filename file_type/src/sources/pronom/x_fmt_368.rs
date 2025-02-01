use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_368: FileFormat = FileFormat {
    id: 538,
    puid: "x-fmt/368",
    name: "VisiCalc Database",
    extensions: &["dif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
