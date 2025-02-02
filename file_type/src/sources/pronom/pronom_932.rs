use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_932: FileFormat = FileFormat {
    id: 932,
    source_type: SourceType::Pronom,
    name: "Structured Query Language Data",
    extensions: &["sql"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
