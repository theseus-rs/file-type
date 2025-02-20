use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1465: FileType = FileType {
    file_format: &FileFormat {
        id: 1_465,
        source_type: SourceType::Pronom,
        name: "ART image format",
        extensions: &["art"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4A, 0x47]),
                        Token::Any(&[&[Token::Literal(&[0x03])], &[Token::Literal(&[0x04])]]),
                        Token::Literal(&[0x0E]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
