use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1457: FileFormat = FileFormat {
    id: 1_457,
    source_type: SourceType::Pronom,
    name: "Cypher Query Language",
    extensions: &["cql"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
