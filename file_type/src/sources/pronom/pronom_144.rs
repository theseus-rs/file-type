use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_144: FileFormat = FileFormat {
    id: 144,
    source_type: SourceType::Pronom,
    name: "Pocket Word Template",
    extensions: &["pwt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
