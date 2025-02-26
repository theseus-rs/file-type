use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_3891: FileType = FileType {
    file_format: &FileFormat {
        id: 3_891,
        source_type: SourceType::Pronom,
        name: "GraphPad Prism",
        extensions: &["pzfx"],
        media_types: &["application/x-graphpad-prism-pzfx"],
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
                        Token::WildcardCountRange(2, 32),
                        Token::Literal(&[
                            0x3C, 0x47, 0x72, 0x61, 0x70, 0x68, 0x50, 0x61, 0x64, 0x50, 0x72, 0x69,
                            0x73, 0x6D, 0x46, 0x69, 0x6C, 0x65,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 638,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_364,
            },
        ],
    },
};
