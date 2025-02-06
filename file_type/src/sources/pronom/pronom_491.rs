use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_491: FileFormat = FileFormat {
    id: 491,
    source_type: SourceType::Pronom,
    name: "InterBase Database",
    extensions: &["gdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
