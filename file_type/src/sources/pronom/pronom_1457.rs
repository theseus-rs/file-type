use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1457: FileFormat = FileFormat {
    id: 1_457,
    source_type: SourceType::Pronom,
    name: "Cypher Query Language",
    extensions: &["cql"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
