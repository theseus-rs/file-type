use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1441: FileFormat = FileFormat {
    id: 2_259,
    puid: "fmt/1441",
    name: "Apple iWork Document",
    extensions: &["iwa", "key", "pages", "numbers", "template"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
