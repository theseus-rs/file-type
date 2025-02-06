use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_144: FileFormat = FileFormat {
    id: 144,
    source_type: SourceType::Pronom,
    name: "Pocket Word Template",
    extensions: &["pwt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
