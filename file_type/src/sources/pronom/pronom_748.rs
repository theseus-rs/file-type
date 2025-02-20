use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_748: FileType = FileType {
    file_format: &FileFormat {
        id: 748,
        source_type: SourceType::Pronom,
        name: "OpenOffice Draw",
        extensions: &["sxd"],
        media_types: &["application/vnd.sun.xml.draw"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00,
                        ]),
                        Token::WildcardCount(20),
                        Token::Literal(&[
                            0x6D, 0x69, 0x6D, 0x65, 0x74, 0x79, 0x70, 0x65, 0x61, 0x70, 0x70, 0x6C,
                            0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x2F, 0x76, 0x6E, 0x64, 0x2E,
                            0x73, 0x75, 0x6E, 0x2E, 0x78, 0x6D, 0x6C, 0x2E, 0x64, 0x72, 0x61, 0x77,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00,
                        ]),
                        Token::WildcardCount(20),
                        Token::Literal(&[
                            0x6D, 0x65, 0x74, 0x61, 0x2E, 0x78, 0x6D, 0x6C, 0x3C, 0x3F, 0x78, 0x6D,
                            0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31,
                            0x2E, 0x30, 0x22,
                        ]),
                        Token::WildcardCount(383),
                        Token::Literal(&[
                            0x6F, 0x66, 0x66, 0x69, 0x63, 0x65, 0x3A, 0x76, 0x65, 0x72, 0x73, 0x69,
                            0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x3E,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 382,
        }],
    },
};
