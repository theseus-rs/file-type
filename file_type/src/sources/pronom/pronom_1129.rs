use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1129: FileType = FileType {
    file_format: &FileFormat {
        id: 1_129,
        source_type: SourceType::Pronom,
        name: "Microsoft Visual FoxPro database container (table files)",
        extensions: &["dbc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x30]),
                        Token::SingleWildcard,
                        Token::Range(&[0x01], &[0x0C]),
                        Token::Range(&[0x01], &[0x1F]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x28, 0x02, 0xA5]),
                        Token::WildcardCount(21),
                        Token::Any(&[
                            &[Token::Range(&[0x41], &[0x5A])],
                            &[Token::Range(&[0x61], &[0x7A])],
                        ]),
                        Token::WildcardCount(10),
                        Token::Any(&[
                            &[Token::Literal(&[0x42])],
                            &[Token::Literal(&[0x43])],
                            &[Token::Literal(&[0x44])],
                            &[Token::Literal(&[0x49])],
                            &[Token::Literal(&[0x4C])],
                            &[Token::Literal(&[0x4D])],
                            &[Token::Literal(&[0x4E])],
                            &[Token::Literal(&[0x50])],
                            &[Token::Literal(&[0x54])],
                            &[Token::Literal(&[0x59])],
                        ]),
                        Token::WildcardCount(508),
                        Token::Literal(&[0x20, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00]),
                        Token::Any(&[
                            &[Token::Range(&[0x41], &[0x5A])],
                            &[Token::Range(&[0x61], &[0x7A])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_121,
        }],
    },
};
