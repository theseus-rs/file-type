use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_45: FileFormat = FileFormat {
    id: 76,
    puid: "x-fmt/45",
    name: "Microsoft Word Document Template",
    extensions: &["dot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 1_554,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 690,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_401,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
