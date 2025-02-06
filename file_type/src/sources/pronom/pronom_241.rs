use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_241: FileFormat = FileFormat {
    id: 241,
    source_type: SourceType::Pronom,
    name: "PHP Script Page",
    extensions: &["php"],
    media_types: &["text/html"],
    signatures: &[],
    related_formats: &[],
};
