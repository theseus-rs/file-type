use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1606: FileType = FileType {
    file_format: &FileFormat {
        id: 1_606,
        source_type: SourceType::Pronom,
        name: "MATLAB Mat File",
        extensions: &["mat", "fig"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x4D, 0x41, 0x54, 0x4C, 0x41, 0x42, 0x20, 0x35, 0x2E, 0x30,
                        ]),
                        Token::WildcardCount(114),
                        Token::Any(&[
                            &[Token::Literal(&[0x00, 0x01, 0x49, 0x4D])],
                            &[Token::Literal(&[0x01, 0x00, 0x4D, 0x49])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 2_375,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_629,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_375,
            },
        ],
    },
};
