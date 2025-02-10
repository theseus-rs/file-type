use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1126: FileType = FileType {
    file_format: &FileFormat {
        id: 1_126,
        source_type: SourceType::Pronom,
        name: "Microsoft Visual FoxPro Class Library",
        extensions: &["vcx"],
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
                        Token::WildcardCount(28),
                        Token::Literal(&[
                            0x50, 0x4C, 0x41, 0x54, 0x46, 0x4F, 0x52, 0x4D, 0x00, 0x00, 0x00, 0x43,
                        ]),
                        Token::WildcardCount(20),
                        Token::Literal(&[
                            0x55, 0x4E, 0x49, 0x51, 0x55, 0x45, 0x49, 0x44, 0x00, 0x00, 0x00, 0x43,
                        ]),
                        Token::WildcardCount(52),
                        Token::Literal(&[
                            0x43, 0x4C, 0x41, 0x53, 0x53, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4D,
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
