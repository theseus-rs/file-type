use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1681: FileFormat = FileFormat {
    id: 1_681,
    source_type: SourceType::Pronom,
    name: "Corel Presentation",
    extensions: &["shw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
