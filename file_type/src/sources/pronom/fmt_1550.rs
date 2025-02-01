use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1550: FileFormat = FileFormat {
    id: 2_375,
    puid: "fmt/1550",
    name: "MATLAB Mat File",
    extensions: &["mat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 1_606,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_629,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_606,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
