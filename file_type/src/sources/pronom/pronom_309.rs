use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_309: FileType = FileType {
    file_format: &FileFormat {
        id: 309,
        source_type: SourceType::Pronom,
        name: "ESRI Arc/Info Binary Grid",
        extensions: &["adf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x27, 0x0A, 0xFF, 0xFF]),
                        Token::Any(&[
                            &[Token::Literal(&[0xFC, 0x14])],
                            &[Token::Literal(&[0xFB, 0xF8])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
