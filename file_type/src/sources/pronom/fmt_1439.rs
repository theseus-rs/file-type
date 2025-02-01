use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1439: FileFormat = FileFormat {
    id: 2_257,
    puid: "fmt/1439",
    name: "Apple iWork Pages",
    extensions: &["pages"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
