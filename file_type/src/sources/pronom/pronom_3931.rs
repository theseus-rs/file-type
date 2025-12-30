use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3931: FileType = FileType {
    file_format: &FileFormat {
        id: 3_931,
        source_type: SourceType::Pronom,
        name: "Immersive Audio Model Format",
        extensions: &["iamf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Any(&[&[Token::Literal(&[0xF8])], &[Token::Literal(&[0xFC])]]),
                        Token::Literal(&[0x06, 0x69, 0x61, 0x6D, 0x66]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
