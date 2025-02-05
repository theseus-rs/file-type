use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1690: FileFormat = FileFormat {
    id: 1_690,
    source_type: SourceType::Pronom,
    name: "HTML Components",
    extensions: &["htc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
