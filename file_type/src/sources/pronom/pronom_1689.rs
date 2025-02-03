use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1689: FileFormat = FileFormat {
    id: 1_689,
    source_type: SourceType::Pronom,
    name: "BASIC File",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
