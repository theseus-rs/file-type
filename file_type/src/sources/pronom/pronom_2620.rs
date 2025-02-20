use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2620: FileType = FileType {
    file_format: &FileFormat {
        id: 2_620,
        source_type: SourceType::Pronom,
        name: "GenBank Flat File",
        extensions: &["gb", "gbk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x4C, 0x4F, 0x43, 0x55, 0x53])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x2F, 0x2F]),
                            Token::Any(&[&[Token::Literal(&[0x0A])], &[Token::Literal(&[0x0D])]]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::Variable,
                    offset: None,
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x4F, 0x52, 0x49, 0x47, 0x49, 0x4E])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
