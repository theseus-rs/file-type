use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1119: FileType = FileType {
    file_format: &FileFormat {
        id: 1_119,
        source_type: SourceType::Pronom,
        name: "Earth Resource Satellite Image Header Format",
        extensions: &["ers"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x44, 0x61, 0x74, 0x61, 0x73, 0x65, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65,
                            0x72, 0x20, 0x42, 0x65, 0x67, 0x69, 0x6E,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x44, 0x61, 0x74, 0x61, 0x73, 0x65, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65,
                            0x72, 0x20, 0x45, 0x6E, 0x64, 0x0A,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
