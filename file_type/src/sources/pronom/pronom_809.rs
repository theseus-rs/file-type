use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_809: FileFormat = FileFormat {
    id: 809,
    source_type: SourceType::Pronom,
    name: "Java Language Source Code File",
    extensions: &["java"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
