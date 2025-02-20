use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_660: FileType = FileType {
    file_format: &FileFormat {
        id: 660,
        source_type: SourceType::Pronom,
        name: "MPEG-2 Program Stream",
        extensions: &["mpeg", "mpg", "mod"],
        media_types: &["video/mpeg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x01, 0xBA]),
                        Token::WildcardCountRange(8, 12),
                        Token::Literal(&[0x00, 0x00, 0x01, 0xBB]),
                        Token::WildcardCountRange(8, 65_536),
                        Token::Literal(&[0x00, 0x00, 0x01, 0xB3]),
                        Token::WildcardCountRange(8, 136),
                        Token::Literal(&[0x00, 0x00, 0x01, 0xB5]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::CanContain,
                id: 1_439,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_207,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 659,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 1_207,
            },
        ],
    },
};
