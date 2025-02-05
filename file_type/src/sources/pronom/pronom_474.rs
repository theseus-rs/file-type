use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_474: FileFormat = FileFormat {
    id: 474,
    source_type: SourceType::Pronom,
    name: "Document Type Definition",
    extensions: &["dtd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
