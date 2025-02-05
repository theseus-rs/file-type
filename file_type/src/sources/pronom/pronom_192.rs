use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_192: FileFormat = FileFormat {
    id: 192,
    source_type: SourceType::Pronom,
    name: "Speller Exclude Dictionary",
    extensions: &["dic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
