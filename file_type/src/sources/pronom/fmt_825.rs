use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_825: FileFormat = FileFormat {
    id: 1_626,
    puid: "fmt/825",
    name: "Apple iWork Numbers",
    extensions: &["numbers"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
