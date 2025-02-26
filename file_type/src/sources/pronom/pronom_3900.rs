use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3900: FileType = FileType {
    file_format: &FileFormat {
        id: 3_900,
        source_type: SourceType::Pronom,
        name: "Codebook Exchange Format",
        extensions: &["qdc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(38),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x3C, 0x43, 0x6F, 0x64, 0x65, 0x42, 0x6F, 0x6F, 0x6B, 0x20, 0x78, 0x6D,
                            0x6C, 0x6E, 0x73, 0x3D, 0x22, 0x75, 0x72, 0x6E, 0x3A, 0x51, 0x44, 0x41,
                            0x2D, 0x58, 0x4D, 0x4C, 0x3A, 0x63, 0x6F, 0x64, 0x65, 0x62, 0x6F, 0x6F,
                            0x6B, 0x3A, 0x31, 0x3A, 0x30, 0x22,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x3C, 0x2F, 0x43, 0x6F, 0x64, 0x65, 0x42, 0x6F, 0x6F, 0x6B, 0x3E,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
