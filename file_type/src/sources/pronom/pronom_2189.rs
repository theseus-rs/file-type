use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2189: FileFormat = FileFormat {
    id: 2_189,
    source_type: SourceType::Pronom,
    name: "OmniPage Pro Document",
    extensions: &["opd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 2_190,
    }],
};
