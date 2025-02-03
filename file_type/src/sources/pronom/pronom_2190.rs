use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2190: FileFormat = FileFormat {
    id: 2_190,
    source_type: SourceType::Pronom,
    name: "OmniPage Document",
    extensions: &["opd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasLowerPriorityThan,
        id: 2_189,
    }],
};
