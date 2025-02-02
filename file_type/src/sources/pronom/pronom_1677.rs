use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1677: FileFormat = FileFormat {
    id: 1_677,
    source_type: SourceType::Pronom,
    name: "Notation3",
    extensions: &["n3"],
    media_types: &["text/n3"],
    internal_signatures: &[],
    related_formats: &[],
};
