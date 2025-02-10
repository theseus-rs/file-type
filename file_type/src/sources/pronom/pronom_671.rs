use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_671: FileType = FileType {
    file_format: &FileFormat {
        id: 671,
        source_type: SourceType::Pronom,
        name: "Still Picture Interchange File Format",
        extensions: &["spf", "jpg"],
        media_types: &["image/jpeg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0xFF, 0xD8, 0xFF, 0xE8, 0x00, 0x20, 0x53, 0x50, 0x49, 0x46, 0x46, 0x00,
                            0x01, 0x00,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00])],
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x03])],
                            &[Token::Literal(&[0x04])],
                        ]),
                        Token::WildcardCount(11),
                        Token::Any(&[
                            &[Token::Literal(&[0x00])],
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x03])],
                            &[Token::Literal(&[0x04])],
                            &[Token::Literal(&[0x05])],
                        ]),
                        Token::WildcardCount(9),
                        Token::Literal(&[0xFF, 0xE8]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 670,
        }],
    },
};
