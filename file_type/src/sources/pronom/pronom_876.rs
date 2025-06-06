use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_876: FileType = FileType {
    file_format: &FileFormat {
        id: 876,
        source_type: SourceType::Pronom,
        name: "SIARD (Software-Independent Archiving of Relational Databases)",
        extensions: &["siard"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x50, 0x4B, 0x03, 0x04]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x50, 0x4B, 0x03, 0x04]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70,
                                0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x62, 0x61, 0x72, 0x2E,
                                0x61, 0x64, 0x6D, 0x69, 0x6E, 0x2E, 0x63, 0x68, 0x2F, 0x78, 0x6D,
                                0x6C, 0x6E, 0x73, 0x2F, 0x73, 0x69, 0x61, 0x72, 0x64, 0x2F, 0x31,
                                0x2E, 0x30, 0x2F, 0x6D, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61,
                                0x2E, 0x78, 0x73, 0x64, 0x22,
                            ]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x50, 0x4B, 0x01]),
                            Token::WildcardCountRange(43, 65_531),
                            Token::Literal(&[0x50, 0x4B, 0x05, 0x06]),
                            Token::WildcardCountRange(18, 65_531),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 382,
        }],
    },
};
