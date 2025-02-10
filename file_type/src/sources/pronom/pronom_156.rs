use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_156: FileType = FileType {
    file_format: &FileFormat {
        id: 156,
        source_type: SourceType::Pronom,
        name: "STL (Standard Tessellation Language) ASCII",
        extensions: &["stl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Any(&[&[Token::Literal(&[0x73])], &[Token::Literal(&[0x20, 0x73])]]),
                        Token::Literal(&[0x6F, 0x6C, 0x69, 0x64, 0x20]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x66, 0x61, 0x63, 0x65, 0x74, 0x20, 0x6E, 0x6F, 0x72, 0x6D, 0x61, 0x6C,
                            0x20,
                        ]),
                        Token::WildcardCountRange(0, 200),
                        Token::Literal(&[
                            0x6F, 0x75, 0x74, 0x65, 0x72, 0x20, 0x6C, 0x6F, 0x6F, 0x70,
                        ]),
                        Token::WildcardCountRange(0, 20),
                        Token::Literal(&[0x76, 0x65, 0x72, 0x74, 0x65, 0x78, 0x20]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
