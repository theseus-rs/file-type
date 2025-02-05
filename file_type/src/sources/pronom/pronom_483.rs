use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_483: FileFormat = FileFormat {
    id: 483,
    source_type: SourceType::Pronom,
    name: "Framework Database",
    extensions: &["fw", "fw2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
