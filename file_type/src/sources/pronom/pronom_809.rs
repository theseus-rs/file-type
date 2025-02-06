use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_809: FileFormat = FileFormat {
    id: 809,
    source_type: SourceType::Pronom,
    name: "Java Language Source Code File",
    extensions: &["java"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
