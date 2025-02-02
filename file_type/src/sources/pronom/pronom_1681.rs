use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1681: FileFormat = FileFormat {
    id: 1_681,
    source_type: SourceType::Pronom,
    name: "Corel Presentation",
    extensions: &["shw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
