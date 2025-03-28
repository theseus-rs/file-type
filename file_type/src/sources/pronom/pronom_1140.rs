use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1140: FileType = FileType {
    file_format: &FileFormat {
        id: 1_140,
        source_type: SourceType::Pronom,
        name: "MrSID Image Format (Multi-resolution Seamless Image Database)",
        extensions: &["sid"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x6D, 0x73, 0x69, 0x64])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xFF, 0xD2, 0xFF, 0xA1])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
