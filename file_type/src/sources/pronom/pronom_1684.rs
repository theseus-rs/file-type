use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1684: FileFormat = FileFormat {
    id: 1_684,
    source_type: SourceType::Pronom,
    name: "JSON-LD",
    extensions: &["jsonld"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
