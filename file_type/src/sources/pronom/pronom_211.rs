use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_211: FileType = FileType {
    file_format: &FileFormat {
        id: 211,
        source_type: SourceType::Pronom,
        name: "Visual FoxPro Database Container File",
        extensions: &["dcx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x0A, 0x00,
                        ]),
                        Token::NotLiteral(&[0x00]),
                        Token::WildcardCount(1_009),
                        Token::Literal(&[
                            0x03, 0x00, 0x02, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                        ]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x01, 0xFF, 0xFF]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x0F, 0x0F]),
                        Token::WildcardCount(491),
                        Token::Range(&[0x41], &[0x5A]),
                        Token::Literal(&[0x00, 0x0A]),
                        Token::WildcardCount(502),
                        Token::Literal(&[0x2B, 0x00]),
                        Token::Range(&[0x0B], &[0x0F]),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x2B, 0x00]),
                        Token::WildcardCount(512),
                        Token::NotLiteral(&[0x00]),
                        Token::Literal(&[0x00]),
                        Token::NotLiteral(&[0x00]),
                        Token::Literal(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_122,
        }],
    },
};
