use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2422: FileFormat = FileFormat {
    id: 2_422,
    source_type: SourceType::Pronom,
    name: "Canon Raw",
    extensions: &["cr3"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x66, 0x74, 0x79, 0x70, 0x63, 0x72, 0x78, 0x20]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x6D, 0x6F, 0x6F, 0x76]),
                    Token::WildcardCount(32),
                    Token::Literal(&[0x43]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_421,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 924,
        },
    ],
};
