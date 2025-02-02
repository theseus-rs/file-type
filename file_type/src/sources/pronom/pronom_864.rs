use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_864: FileFormat = FileFormat {
    id: 864,
    source_type: SourceType::Pronom,
    name: "Steel Detailing Neutral Format",
    extensions: &["sdn"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
