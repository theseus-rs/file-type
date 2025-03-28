use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1316: FileType = FileType {
    file_format: &FileFormat {
        id: 1_316,
        source_type: SourceType::Pronom,
        name: "JPEG Network Graphics",
        extensions: &["jng"],
        media_types: &["image/x-jng"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x8B, 0x4A, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x10,
                            0x4A, 0x48, 0x44, 0x52,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
