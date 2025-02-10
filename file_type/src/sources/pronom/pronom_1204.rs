use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1204: FileType = FileType {
    file_format: &FileFormat {
        id: 1_204,
        source_type: SourceType::Pronom,
        name: "Adobe Illustrator",
        extensions: &["ai", "eps"],
        media_types: &["application/postscript"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x33,
                            0x2E, 0x30,
                        ]),
                        Token::WildcardCountRange(128, 1_024),
                        Token::Literal(&[
                            0x25, 0x41, 0x49, 0x35, 0x5F, 0x46, 0x69, 0x6C, 0x65, 0x46, 0x6F, 0x72,
                            0x6D, 0x61, 0x74, 0x20, 0x32,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 86,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 331,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 332,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 771,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 773,
            },
        ],
    },
};
