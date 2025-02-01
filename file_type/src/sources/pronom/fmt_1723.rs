use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1723: FileFormat = FileFormat {
    id: 2_567,
    puid: "fmt/1723",
    name: "Wordcraft Chapter Files",
    extensions: &["001"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
