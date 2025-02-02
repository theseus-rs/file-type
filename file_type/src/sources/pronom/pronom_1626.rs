use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1626: FileFormat = FileFormat {
    id: 1_626,
    source_type: SourceType::Pronom,
    name: "Apple iWork Numbers",
    extensions: &["numbers"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
