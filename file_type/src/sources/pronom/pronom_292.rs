use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_292: FileFormat = FileFormat {
    id: 292,
    source_type: SourceType::Pronom,
    name: "XYWrite Document",
    extensions: &["xy3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
