use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_491: FileFormat = FileFormat {
    id: 491,
    source_type: SourceType::Pronom,
    name: "InterBase Database",
    extensions: &["gdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
