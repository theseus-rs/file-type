use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1244: FileType = FileType {
    file_format: &FileFormat {
        id: 1_244,
        source_type: SourceType::Pronom,
        name: "Verity Collection Document Dataset Descriptor Style Set",
        extensions: &["ddd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x23, 0x20, 0x73, 0x74, 0x79, 0x6C, 0x65, 0x2E, 0x64, 0x64, 0x64, 0x09,
                            0x31, 0x2E, 0x37, 0x20, 0x2D, 0x20, 0x31, 0x2F, 0x31, 0x38, 0x2F, 0x39,
                            0x34, 0x0D, 0x0A, 0x23, 0x20, 0x40, 0x28, 0x23, 0x29,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x7D, 0x0D, 0x0A, 0x0D, 0x0A, 0x0D, 0x0A, 0x7D, 0x0D, 0x0A,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
