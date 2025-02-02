use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_291: FileFormat = FileFormat {
    id: 291,
    source_type: SourceType::Pronom,
    name: "XYWrite Document",
    extensions: &["xy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
