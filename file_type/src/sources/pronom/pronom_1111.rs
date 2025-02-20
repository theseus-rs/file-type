use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1111: FileType = FileType {
    file_format: &FileFormat {
        id: 1_111,
        source_type: SourceType::Pronom,
        name: "National Imagery Transmission Format",
        extensions: &["ntf"],
        media_types: &["application/vnd.nitf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4E, 0x49, 0x54, 0x46, 0x30, 0x31, 0x2E, 0x31, 0x30]),
                        Token::WildcardCount(16),
                        Token::Range(&[0x30], &[0x33]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Range(&[0x30], &[0x32]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Range(&[0x30], &[0x35]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Range(&[0x30], &[0x35]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Literal(&[0x5A]),
                        Token::WildcardCount(3),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::WildcardCount(80),
                        Token::Any(&[
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x52])],
                            &[Token::Literal(&[0x53])],
                            &[Token::Literal(&[0x54])],
                            &[Token::Literal(&[0x55])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_112,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_113,
            },
        ],
    },
};
