use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1619: FileFormat = FileFormat {
    id: 2_446,
    puid: "fmt/1619",
    name: "Pascal Source Code",
    extensions: &["pas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
