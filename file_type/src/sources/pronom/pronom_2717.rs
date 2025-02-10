use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2717: FileType = FileType {
    file_format: &FileFormat {
        id: 2_717,
        source_type: SourceType::Pronom,
        name: "Adobe Illustrator CC 2020 Artwork",
        extensions: &["ai", "ait"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x35]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x3C, 0x69, 0x6C, 0x6C, 0x75, 0x73, 0x74, 0x72, 0x61, 0x74, 0x6F, 0x72,
                            0x3A, 0x54, 0x79, 0x70, 0x65, 0x3E, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65,
                            0x6E, 0x74, 0x3C, 0x2F, 0x69, 0x6C, 0x6C, 0x75, 0x73, 0x74, 0x72, 0x61,
                            0x74, 0x6F, 0x72, 0x3A, 0x54, 0x79, 0x70, 0x65, 0x3E,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x33,
                            0x2E, 0x30,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x25, 0x41, 0x49, 0x35, 0x5F, 0x46, 0x69, 0x6C, 0x65, 0x46, 0x6F, 0x72,
                            0x6D, 0x61, 0x74, 0x20, 0x31, 0x34,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 618,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_718,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_716,
            },
        ],
    },
};
