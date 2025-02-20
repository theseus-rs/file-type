use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1393: FileType = FileType {
    file_format: &FileFormat {
        id: 1_393,
        source_type: SourceType::Pronom,
        name: "Statistical Analysis System Catalogue XPT (Windows)",
        extensions: &["xpt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(92),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x58, 0x50, 0x5F, 0x50, 0x52, 0x4F]),
                            Token::WildcardCountRange(0, 16),
                            Token::Literal(&[0x53, 0x41, 0x53, 0x39, 0x2E, 0x31]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x45, 0x6E, 0x64, 0x20, 0x6F, 0x66, 0x20, 0x4F, 0x62, 0x6A, 0x65, 0x63,
                            0x74,
                        ])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
