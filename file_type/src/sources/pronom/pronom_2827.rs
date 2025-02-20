use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2827: FileType = FileType {
    file_format: &FileFormat {
        id: 2_827,
        source_type: SourceType::Pronom,
        name: "SolidWorks Material Database File",
        extensions: &["sldmat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                            0x6E, 0x3D,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::Literal(&[0x31, 0x2E, 0x30]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::WildcardCountRange(3, 24),
                        Token::Literal(&[
                            0x3C, 0x6D, 0x73, 0x74, 0x6E, 0x73, 0x3A, 0x6D, 0x61, 0x74, 0x65, 0x72,
                            0x69, 0x61, 0x6C, 0x73, 0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x6D,
                            0x73, 0x74, 0x6E, 0x73, 0x3D,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::Literal(&[
                            0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x73,
                            0x6F, 0x6C, 0x69, 0x64, 0x77, 0x6F, 0x72, 0x6B, 0x73, 0x2E, 0x63, 0x6F,
                            0x6D, 0x2F, 0x73, 0x6C, 0x64, 0x6D, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61,
                            0x6C, 0x73,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        }],
    },
};
