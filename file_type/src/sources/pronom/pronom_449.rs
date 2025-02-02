use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_449: FileFormat = FileFormat {
    id: 449,
    source_type: SourceType::Pronom,
    name: "Micrografx Designer",
    extensions: &["drw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasLowerPriorityThan,
        id: 447,
    }],
};
