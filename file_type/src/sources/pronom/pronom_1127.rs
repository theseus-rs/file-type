use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1127: FileType = FileType {
    file_format: &FileFormat {
        id: 1_127,
        source_type: SourceType::Pronom,
        name: "Microsoft Visual FoxPro Project",
        extensions: &["pjx"],
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
                        Token::Literal(&[0xA8, 0x04, 0x82]),
                        Token::WildcardCount(21),
                        Token::Literal(&[
                            0x4E, 0x41, 0x4D, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4D,
                        ]),
                        Token::WildcardCount(20),
                        Token::Literal(&[
                            0x54, 0x59, 0x50, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43,
                        ]),
                        Token::WildcardCount(20),
                        Token::Literal(&[
                            0x49, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4E,
                        ]),
                        Token::WildcardCount(20),
                        Token::Literal(&[
                            0x54, 0x49, 0x4D, 0x45, 0x53, 0x54, 0x41, 0x4D, 0x50, 0x00, 0x00, 0x4E,
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
