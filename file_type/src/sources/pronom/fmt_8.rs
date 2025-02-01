use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_8: FileFormat = FileFormat {
    id: 610,
    puid: "fmt/8",
    name: "Tagged Image File Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 672,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 673,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 752,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
