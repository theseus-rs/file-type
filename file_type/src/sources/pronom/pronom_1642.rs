use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1642: FileType = FileType {
    file_format: &FileFormat {
        id: 1_642,
        source_type: SourceType::Pronom,
        name: "Interleaved ADX Audio Format (AIX)",
        extensions: &["aix"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x49, 0x58, 0x46, 0x00, 0x00, 0x17, 0xF8, 0x01, 0x00, 0x00, 0x14,
                        0x00, 0x00, 0x08, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
