use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2584: FileType = FileType {
    file_format: &FileFormat {
        id: 2_584,
        source_type: SourceType::Pronom,
        name: "UDF Disc Image",
        extensions: &["toast", "iso", "cdr", "dmg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(32_768),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x42, 0x45, 0x41, 0x30, 0x31, 0x01]),
                        Token::WildcardCount(2_041),
                        Token::Literal(&[0x00, 0x4E, 0x53, 0x52, 0x30]),
                        Token::Any(&[&[Token::Literal(&[0x32])], &[Token::Literal(&[0x33])]]),
                        Token::Literal(&[0x01]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_585,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_605,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 2_585,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 1_255,
            },
        ],
    },
};
