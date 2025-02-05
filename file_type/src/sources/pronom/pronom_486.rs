use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_486: FileFormat = FileFormat {
    id: 486,
    source_type: SourceType::Pronom,
    name: "Framework Database",
    extensions: &["fw4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
