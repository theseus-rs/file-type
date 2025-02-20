use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1819: FileType = FileType {
    file_format: &FileFormat {
        id: 1_819,
        source_type: SourceType::Pronom,
        name: "INTERLIS Model File",
        extensions: &["ili"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x54, 0x52, 0x41, 0x4E, 0x53, 0x46, 0x45, 0x52]),
                            Token::WildcardCountRange(1, 1_024),
                            Token::Literal(&[0x4D, 0x4F, 0x44, 0x45, 0x4C]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54]),
                            Token::WildcardCountRange(1, 124),
                            Token::Literal(&[0x43, 0x4F, 0x44, 0x45]),
                            Token::WildcardCountRange(1, 124),
                            Token::Literal(&[0x45, 0x4E, 0x44]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
