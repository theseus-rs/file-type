use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1298: FileType = FileType {
    file_format: &FileFormat {
        id: 1_298,
        source_type: SourceType::Pronom,
        name: "PowerProject",
        extensions: &["pp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x00, 0x00, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x37, 0x30, 0x30,
                            0x36, 0x20,
                        ]),
                        Token::WildcardCount(13),
                        Token::Literal(&[
                            0x2F, 0x2F, 0x64, 0x6F, 0x64, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                            0x6F, 0x6E, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                            0x69, 0x64, 0x0D, 0x0A,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_299,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_300,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_301,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_302,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_303,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_304,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_297,
            },
        ],
    },
};
