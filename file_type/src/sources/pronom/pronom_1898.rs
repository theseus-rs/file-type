use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1898: FileFormat = FileFormat {
    id: 1_898,
    source_type: SourceType::Pronom,
    name: "Exclude File",
    extensions: &["exclude"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
