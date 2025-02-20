use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_786: FileType = FileType {
    file_format: &FileFormat {
        id: 786,
        source_type: SourceType::Pronom,
        name: "Waveform Audio (WAVEFORMATEXTENSIBLE)",
        extensions: &["wav", "wave"],
        media_types: &["audio/x-wav"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x57, 0x41, 0x56, 0x45, 0x66, 0x6D, 0x74, 0x20]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0xFE, 0xFF]),
                        Token::WildcardCountRange(38, 4_294_967_295),
                        Token::Literal(&[0x64, 0x61, 0x74, 0x61]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 656,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 735,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_314,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_508,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_509,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_510,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 654,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 785,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 785,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 654,
            },
        ],
    },
};
