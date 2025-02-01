use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_296: FileFormat = FileFormat {
    id: 449,
    puid: "x-fmt/296",
    name: "Micrografx Designer",
    extensions: &["drw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 447,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
