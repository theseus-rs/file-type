use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2106: FileType = FileType {
    file_format: &FileFormat {
        id: 2_106,
        source_type: SourceType::Pronom,
        name: "IESNA LM-63 Photometric Data File",
        extensions: &["ies"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x45, 0x53, 0x4E, 0x41]),
                        Token::Any(&[
                            &[Token::Literal(&[0x39, 0x31])],
                            &[Token::Literal(&[0x3A, 0x4C, 0x4D, 0x2D, 0x36, 0x33, 0x2D])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
