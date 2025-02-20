use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_154: FileType = FileType {
    file_format: &FileFormat {
        id: 154,
        source_type: SourceType::Pronom,
        name: "Microsoft Symbolic Link (SYLK) File",
        extensions: &["slk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x49, 0x44, 0x3B, 0x50])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Any(&[
                                &[Token::Literal(&[0x0D])],
                                &[Token::Literal(&[0x0D, 0x0A])],
                                &[Token::Literal(&[0x0A])],
                            ]),
                            Token::Literal(&[0x45]),
                            Token::Any(&[
                                &[Token::Literal(&[0x0D])],
                                &[Token::Literal(&[0x0D, 0x0A])],
                                &[Token::Literal(&[0x0A])],
                            ]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
