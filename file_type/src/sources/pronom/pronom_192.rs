use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_192: FileFormat = FileFormat {
    id: 192,
    source_type: SourceType::Pronom,
    name: "Speller Exclude Dictionary",
    extensions: &["dic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
