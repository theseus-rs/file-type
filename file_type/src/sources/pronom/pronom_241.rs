use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_241: FileFormat = FileFormat {
    id: 241,
    source_type: SourceType::Pronom,
    name: "PHP Script Page",
    extensions: &["php"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
