use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_291: FileFormat = FileFormat {
    id: 291,
    source_type: SourceType::Pronom,
    name: "XYWrite Document",
    extensions: &["xy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
