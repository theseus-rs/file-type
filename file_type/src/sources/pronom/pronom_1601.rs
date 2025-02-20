use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1601: FileType = FileType {
    file_format: &FileFormat {
        id: 1_601,
        source_type: SourceType::Pronom,
        name: "TAP (ZX Spectrum)",
        extensions: &["tap"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x13, 0x00, 0x00]),
                        Token::WildcardCount(20),
                        Token::Literal(&[0xFF]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
