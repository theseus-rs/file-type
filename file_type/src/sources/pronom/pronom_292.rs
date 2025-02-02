use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_292: FileFormat = FileFormat {
    id: 292,
    source_type: SourceType::Pronom,
    name: "XYWrite Document",
    extensions: &["xy3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
