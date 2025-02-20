use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1124: FileType = FileType {
    file_format: &FileFormat {
        id: 1_124,
        source_type: SourceType::Pronom,
        name: "Microsoft Visual FoxPro Report",
        extensions: &["frx"],
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
                        Token::WildcardCount(1),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x88, 0x0A, 0xE5]),
                        Token::WildcardCount(21),
                        Token::Literal(&[
                            0x50, 0x4C, 0x41, 0x54, 0x46, 0x4F, 0x52, 0x4D, 0x00, 0x00, 0x00, 0x43,
                        ]),
                        Token::WildcardCount(20),
                        Token::Literal(&[
                            0x55, 0x4E, 0x49, 0x51, 0x55, 0x45, 0x49, 0x44, 0x00, 0x00, 0x00, 0x43,
                        ]),
                        Token::WildcardCount(52),
                        Token::Literal(&[
                            0x4F, 0x42, 0x4A, 0x54, 0x59, 0x50, 0x45, 0x00, 0x00, 0x00, 0x00, 0x4E,
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
