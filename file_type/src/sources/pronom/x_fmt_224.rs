use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_224: FileFormat = FileFormat {
    id: 316,
    puid: "x-fmt/224",
    name: "Cascading Style Sheet",
    extensions: &["css"],
    media_types: &["text/css"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 635,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
