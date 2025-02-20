use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_448: FileType = FileType {
    file_format: &FileFormat {
        id: 448,
        source_type: SourceType::Pronom,
        name: "Micrografx Draw",
        extensions: &["drw", "drt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x01, 0xFF, 0x02, 0x04, 0x03, 0x02, 0x00, 0x02]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x02])]]),
                        Token::Literal(&[0x02, 0x21, 0x05]),
                        Token::WildcardCount(3),
                        Token::Literal(&[
                            0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x20, 0x44, 0x72, 0x61, 0x77,
                            0x20, 0x44, 0x72, 0x61, 0x77, 0x69, 0x6E, 0x67,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 78,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 447,
            },
        ],
    },
};
