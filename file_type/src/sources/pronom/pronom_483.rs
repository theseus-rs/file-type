use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_483: FileFormat = FileFormat {
    id: 483,
    source_type: SourceType::Pronom,
    name: "Framework Database",
    extensions: &["fw", "fw2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
