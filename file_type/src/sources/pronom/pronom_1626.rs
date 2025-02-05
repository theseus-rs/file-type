use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1626: FileFormat = FileFormat {
    id: 1_626,
    source_type: SourceType::Pronom,
    name: "Apple iWork Numbers",
    extensions: &["numbers"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
