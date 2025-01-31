use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1440: FileFormat = FileFormat {
    id: 2_258,
    puid: "fmt/1440",
    name: "Apple iWork Numbers",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
