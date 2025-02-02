use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1684: FileFormat = FileFormat {
    id: 1_684,
    source_type: SourceType::Pronom,
    name: "JSON-LD",
    extensions: &["jsonld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
