use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_549: FileFormat = FileFormat {
    id: 549,
    source_type: SourceType::Pronom,
    name: "XYWrite Document",
    extensions: &["xy4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
