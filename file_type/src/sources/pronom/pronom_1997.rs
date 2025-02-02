use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1997: FileFormat = FileFormat {
    id: 1_997,
    source_type: SourceType::Pronom,
    name: "Apple iWork Template",
    extensions: &["template"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
