use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1690: FileFormat = FileFormat {
    id: 1_690,
    source_type: SourceType::Pronom,
    name: "HTML Components",
    extensions: &["htc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
