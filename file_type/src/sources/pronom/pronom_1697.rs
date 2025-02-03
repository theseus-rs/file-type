use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1697: FileFormat = FileFormat {
    id: 1_697,
    source_type: SourceType::Pronom,
    name: "i2 Analysts Notebook",
    extensions: &["anb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
