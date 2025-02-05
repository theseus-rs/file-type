use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1689: FileFormat = FileFormat {
    id: 1_689,
    source_type: SourceType::Pronom,
    name: "BASIC File",
    extensions: &["bas"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
