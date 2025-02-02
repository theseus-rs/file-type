use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2611: FileFormat = FileFormat {
    id: 2_611,
    source_type: SourceType::Pronom,
    name: "MacBinary",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 2_612,
    }],
};
