use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2854: FileType = FileType {
    file_format: &FileFormat {
        id: 2_854,
        source_type: SourceType::Pronom,
        name: "Sibelius Score",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x0F, 0x53, 0x49, 0x42, 0x45, 0x4C, 0x49, 0x55, 0x53]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x00, 0x2D]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_495,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_855,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_852,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 1_495,
            },
        ],
    },
};
