use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2747: FileType = FileType {
    file_format: &FileFormat {
        id: 2_747,
        source_type: SourceType::Pronom,
        name: "Digital Voice File (DVF)",
        extensions: &["dvf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x53, 0x5F, 0x56, 0x4F, 0x49, 0x43, 0x45]),
                        Token::WildcardCount(4),
                        Token::Literal(&[
                            0x01, 0x02, 0x00, 0x00, 0x53, 0x4F, 0x4E, 0x59, 0x20, 0x43, 0x4F, 0x52,
                            0x50, 0x4F, 0x52, 0x41, 0x54, 0x49, 0x4F, 0x4E,
                        ]),
                        Token::WildcardCount(28),
                        Token::Literal(&[0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x35])],
                            &[Token::Literal(&[0x37])],
                        ]),
                        Token::Literal(&[0x00, 0x01]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_259,
        }],
    },
};
