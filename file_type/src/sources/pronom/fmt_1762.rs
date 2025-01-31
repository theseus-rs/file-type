use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1762: FileFormat = FileFormat {
    id: 2_612,
    puid: "fmt/1762",
    name: "MacBinary",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(73),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(7),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(16),
                    Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x81,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_613,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_611,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
