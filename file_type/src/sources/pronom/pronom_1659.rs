use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1659: FileType = FileType {
    file_format: &FileFormat {
        id: 1_659,
        source_type: SourceType::Pronom,
        name: "Navisworks Document",
        extensions: &["nwd", "nwc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x23, 0x4C, 0x63, 0x55, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6D, 0x2D, 0x56,
                            0x31, 0x2E, 0x31, 0x20, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x20, 0x64,
                            0x6F, 0x75, 0x62, 0x6C, 0x65, 0x20, 0x72, 0x65, 0x6C, 0x65, 0x61, 0x73,
                            0x65, 0x20, 0x43, 0x20, 0x6C, 0x69, 0x63, 0x68, 0x75, 0x6E, 0x6B, 0x2D,
                            0x30, 0x30, 0x37,
                        ]),
                        Token::WildcardCountRange(0, 16),
                        Token::Literal(&[
                            0x23, 0x4C, 0x63, 0x55, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6D, 0x2D, 0x56,
                            0x31, 0x2E, 0x31, 0x20, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x20, 0x64,
                            0x6F, 0x75, 0x62, 0x6C, 0x65, 0x20, 0x72, 0x65, 0x6C, 0x65, 0x61, 0x73,
                            0x65, 0x20, 0x75, 0x74, 0x66, 0x38, 0x20, 0x6E, 0x61, 0x76, 0x69, 0x73,
                            0x77, 0x6F, 0x72, 0x6B, 0x73, 0x3A, 0x31, 0x31, 0x34,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_658,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_660,
            },
        ],
    },
};
