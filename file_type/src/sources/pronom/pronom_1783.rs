use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1783: FileFormat = FileFormat {
    id: 1_783,
    source_type: SourceType::Pronom,
    name: "3DS Max",
    extensions: &["max", "chr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasLowerPriorityThan,
        id: 2_292,
    }],
};
