use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_447: FileType = FileType {
    file_format: &FileFormat {
        id: 447,
        source_type: SourceType::Pronom,
        name: "Micrografx Draw",
        extensions: &["drw"],
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
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 448,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 78,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 449,
            },
        ],
    },
};
