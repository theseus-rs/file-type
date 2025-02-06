use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_191: FileFormat = FileFormat {
    id: 191,
    source_type: SourceType::Pronom,
    name: "Speller Custom Dictionary",
    extensions: &["dic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
