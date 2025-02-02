use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_474: FileFormat = FileFormat {
    id: 474,
    source_type: SourceType::Pronom,
    name: "Document Type Definition",
    extensions: &["dtd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
