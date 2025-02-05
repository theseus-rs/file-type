use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_775: FileFormat = FileFormat {
    id: 775,
    source_type: SourceType::Pronom,
    name: "Windows New Executable",
    extensions: &["exe"],
    media_types: &[],
    signatures: &[Signature {
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
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 776,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_704,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_705,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 774,
        },
    ],
};
