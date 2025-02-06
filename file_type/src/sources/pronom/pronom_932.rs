use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_932: FileFormat = FileFormat {
    id: 932,
    source_type: SourceType::Pronom,
    name: "Structured Query Language Data",
    extensions: &["sql"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
