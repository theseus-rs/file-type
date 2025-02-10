use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_311: FileType = FileType {
    file_format: &FileFormat {
        id: 311,
        source_type: SourceType::Pronom,
        name: "Applixware Spreadsheet",
        extensions: &["as"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x2A, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x53, 0x50, 0x52, 0x45, 0x41,
                            0x44, 0x53, 0x48, 0x45, 0x45, 0x54, 0x53, 0x20, 0x56, 0x45, 0x52, 0x53,
                            0x49, 0x4F, 0x4E, 0x3D,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x2A, 0x45, 0x4E, 0x44, 0x20, 0x53, 0x50, 0x52, 0x45, 0x41, 0x44, 0x53,
                            0x48, 0x45, 0x45, 0x54, 0x53, 0x0A,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
