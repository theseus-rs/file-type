use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1246: FileType = FileType {
    file_format: &FileFormat {
        id: 1_246,
        source_type: SourceType::Pronom,
        name: "Verity Collection Word List Descriptor Style Set",
        extensions: &["wld"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x23, 0x20, 0x73, 0x74, 0x79, 0x6C, 0x65, 0x2E, 0x77, 0x6C, 0x64, 0x09,
                            0x31, 0x2E, 0x35, 0x20, 0x2D, 0x20, 0x31, 0x2F, 0x31, 0x31, 0x2F, 0x39,
                            0x34, 0x0D, 0x0A, 0x23, 0x20, 0x40, 0x28, 0x23, 0x29,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x7D, 0x0D, 0x0A, 0x7D, 0x0D, 0x0A, 0x24, 0x24, 0x0D, 0x0A,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
