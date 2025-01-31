use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_385: FileFormat = FileFormat {
    id: 1_133,
    puid: "fmt/385",
    name: "Microsoft Windows Cursor",
    extensions: &["cur"],
    media_types: &["image/x-win-bitmap"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x02, 0x00]),
                        Token::Range(&[0x01], &[0x09]),
                        Token::Literal(&[0x00]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x00]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x00]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x00]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x00]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x00, 0x00, 0x28, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x00]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x00, 0x01, 0x00]),
                        Token::Range(&[0x01], &[0x20]),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x28, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x00]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x00, 0x01, 0x00]),
                        Token::Range(&[0x01], &[0x20]),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[],
};
