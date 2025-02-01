use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_410: FileFormat = FileFormat {
    id: 775,
    puid: "x-fmt/410",
    name: "Windows New Executable",
    extensions: &["exe"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x5A]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x4E, 0x45]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 776,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_704,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_705,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 774,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
