use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2021: FileType = FileType {
    file_format: &FileFormat {
        id: 2_021,
        source_type: SourceType::Pronom,
        name: "Wavefront Material Template Library",
        extensions: &["mtl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x6E, 0x65, 0x77, 0x6D, 0x74, 0x6C]),
                        Token::WildcardCountRange(0, 80),
                        Token::Any(&[
                            &[Token::Literal(&[0x4B, 0x61])],
                            &[Token::Literal(&[0x4B, 0x64])],
                            &[Token::Literal(&[0x4B, 0x73])],
                        ]),
                        Token::Literal(&[0x20]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
