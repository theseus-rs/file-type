use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_731: FileType = FileType {
    file_format: &FileFormat {
        id: 731,
        source_type: SourceType::Pronom,
        name: "Windows Bitmap",
        extensions: &["bmp", "dib"],
        media_types: &["image/bmp"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x42, 0x4D]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x6C, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(8),
                        Token::Literal(&[0x01, 0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x04])],
                            &[Token::Literal(&[0x08])],
                            &[Token::Literal(&[0x10])],
                            &[Token::Literal(&[0x18])],
                            &[Token::Literal(&[0x20])],
                        ]),
                        Token::Literal(&[0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00])],
                            &[Token::Literal(&[0x01])],
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x03])],
                        ]),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 732,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 729,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 730,
            },
        ],
    },
};
