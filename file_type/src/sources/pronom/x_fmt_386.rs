use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_386: FileFormat = FileFormat {
    id: 660,
    puid: "x-fmt/386",
    name: "MPEG-2 Program Stream",
    extensions: &["mpeg", "mpg", "mod"],
    media_types: &["video/mpeg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x01, 0xBA]),
                    Token::WildcardCountRange(8, 12),
                    Token::Literal(&[0x00, 0x00, 0x01, 0xBB]),
                    Token::WildcardCountRange(8, 65_536),
                    Token::Literal(&[0x00, 0x00, 0x01, 0xB3]),
                    Token::WildcardCountRange(8, 136),
                    Token::Literal(&[0x00, 0x00, 0x01, 0xB5]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_439,
            relationship_type: RelationshipType::CanContain,
        },
        RelatedFormat {
            id: 1_207,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 659,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_207,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
