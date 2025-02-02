use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_191: FileFormat = FileFormat {
    id: 191,
    source_type: SourceType::Pronom,
    name: "Speller Custom Dictionary",
    extensions: &["dic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
