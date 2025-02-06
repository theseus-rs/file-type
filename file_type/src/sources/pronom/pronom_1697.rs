use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1697: FileFormat = FileFormat {
    id: 1_697,
    source_type: SourceType::Pronom,
    name: "i2 Analysts Notebook",
    extensions: &["anb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
