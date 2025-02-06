use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1678: FileFormat = FileFormat {
    id: 1_678,
    source_type: SourceType::Pronom,
    name: "Turtle",
    extensions: &["ttl"],
    media_types: &["text/turtle"],
    signatures: &[],
    related_formats: &[],
};
