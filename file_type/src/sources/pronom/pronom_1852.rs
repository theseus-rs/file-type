use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1852: FileType = FileType {
    file_format: &FileFormat {
        id: 1_852,
        source_type: SourceType::Pronom,
        name: "Geography Markup Language",
        extensions: &["gml"],
        media_types: &["application/gml+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                            0x6E, 0x3D,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::Literal(&[0x31, 0x2E, 0x30]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::WildcardCountRange(0, 320),
                        Token::Literal(&[
                            0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x67, 0x6D, 0x6C, 0x3D,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::Literal(&[
                            0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x6F,
                            0x70, 0x65, 0x6E, 0x67, 0x69, 0x73, 0x2E, 0x6E, 0x65, 0x74, 0x2F, 0x67,
                            0x6D, 0x6C, 0x2F, 0x33, 0x2E, 0x32, 0x22,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 429,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 319,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 638,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 319,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 638,
            },
        ],
    },
};
