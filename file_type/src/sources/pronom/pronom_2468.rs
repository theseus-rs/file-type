use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2468: FileType = FileType {
    file_format: &FileFormat {
        id: 2_468,
        source_type: SourceType::Pronom,
        name: "Adobe InDesign Interchange Document",
        extensions: &["inx"],
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
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x3C, 0x3F, 0x61, 0x69, 0x64, 0x20, 0x73, 0x74, 0x79, 0x6C, 0x65, 0x3D,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::Literal(&[0x33, 0x33]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
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
