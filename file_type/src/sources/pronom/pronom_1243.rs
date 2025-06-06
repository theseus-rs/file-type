use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1243: FileType = FileType {
    file_format: &FileFormat {
        id: 1_243,
        source_type: SourceType::Pronom,
        name: "Verity Collection Index Style Policy",
        extensions: &["plc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x23, 0x20, 0x73, 0x74, 0x79, 0x6C, 0x65, 0x2E, 0x70, 0x6C, 0x63, 0x09,
                            0x31, 0x2E, 0x31, 0x31, 0x20, 0x2D, 0x20, 0x33, 0x2F, 0x32, 0x2F, 0x39,
                            0x34, 0x0D, 0x0A, 0x23, 0x20, 0x40, 0x28, 0x23, 0x29, 0x43, 0x6F, 0x70,
                            0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x28, 0x43, 0x29, 0x20, 0x31,
                            0x39, 0x38, 0x37, 0x2D, 0x31, 0x39, 0x39, 0x34, 0x20, 0x56, 0x65, 0x72,
                            0x69, 0x74, 0x79, 0x2C, 0x20, 0x49, 0x6E, 0x63, 0x2E, 0x0D, 0x0A, 0x23,
                            0x0D, 0x0A, 0x23,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x7D, 0x0D, 0x0A, 0x7D, 0x0D, 0x0A])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
