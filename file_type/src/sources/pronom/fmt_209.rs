use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_209: FileFormat = FileFormat {
    id: 935,
    puid: "fmt/209",
    name: "Sound Designer II Audio File",
    extensions: &["sd2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
