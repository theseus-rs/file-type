use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1786: FileType = FileType {
    file_format: &FileFormat {
        id: 1_786,
        source_type: SourceType::Pronom,
        name: "EazyDraw File Format",
        extensions: &["ezdraw"],
        media_types: &[],
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
                        Token::WildcardCountRange(8, 256),
                        Token::Literal(&[
                            0x3C, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                            0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x3E,
                        ]),
                        Token::WildcardCountRange(8, 1_024),
                        Token::Literal(&[
                            0x45, 0x61, 0x7A, 0x79, 0x44, 0x72, 0x61, 0x77, 0x56, 0x65, 0x72, 0x73,
                            0x69, 0x6F, 0x6E,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 638,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_784,
            },
        ],
    },
};
