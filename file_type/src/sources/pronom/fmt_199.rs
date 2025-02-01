use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_199: FileFormat = FileFormat {
    id: 924,
    puid: "fmt/199",
    name: "MPEG-4 Media File",
    extensions: &["mp4", "m4v", "m4a", "f4v", "f4a"],
    media_types: &["application/mp4", "video/mp4"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(4),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x66, 0x74, 0x79, 0x70]),
                    Token::WildcardCountRange(0, 64),
                    Token::Any(&[
                        &[Token::Literal(&[0x6D, 0x70, 0x34, 0x32])],
                        &[Token::Literal(&[0x6D, 0x70, 0x34, 0x31])],
                        &[Token::Literal(&[0x69, 0x73, 0x6F, 0x6D])],
                        &[Token::Literal(&[0x69, 0x73, 0x6F, 0x32])],
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x6D, 0x6F, 0x6F, 0x76]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_103,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_388,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_422,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 658,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_443,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
