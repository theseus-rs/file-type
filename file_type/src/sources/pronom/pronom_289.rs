use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_289: FileType = FileType {
    file_format: &FileFormat {
        id: 289,
        source_type: SourceType::Pronom,
        name: "X-Windows Pixmap Image",
        extensions: &["xpm"],
        media_types: &["image/x-xpixmap"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x2F, 0x2A, 0x20, 0x58, 0x50, 0x4D, 0x20, 0x2A, 0x2F]),
                            Token::AnyWildcard,
                            Token::Literal(&[
                                0x0A, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x63, 0x68, 0x61,
                                0x72, 0x20, 0x2A, 0x20,
                            ]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x7D]),
                            Token::Any(&[
                                &[Token::Literal(&[0x3B])],
                                &[Token::Literal(&[0x20, 0x3B])],
                            ]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
