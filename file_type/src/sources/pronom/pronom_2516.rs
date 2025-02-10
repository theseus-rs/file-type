use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2516: FileType = FileType {
    file_format: &FileFormat {
        id: 2_516,
        source_type: SourceType::Pronom,
        name: "INTREPID Standard Information File",
        extensions: &["isi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x4D, 0x65, 0x74, 0x61, 0x44, 0x61, 0x74, 0x61, 0x20, 0x42, 0x65, 0x67,
                            0x69, 0x6E, 0x0D, 0x0A, 0x09, 0x4E, 0x61, 0x6D, 0x65, 0x09, 0x3D,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x45, 0x6E, 0x64, 0x0D, 0x0A, 0x4D, 0x65, 0x74, 0x61, 0x44, 0x61, 0x74,
                            0x61, 0x20, 0x45, 0x6E, 0x64, 0x0D, 0x0A,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
