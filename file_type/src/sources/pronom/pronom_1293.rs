use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1293: FileType = FileType {
    file_format: &FileFormat {
        id: 1_293,
        source_type: SourceType::Pronom,
        name: "Adobe Flash",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[
            Signature {
                byte_sequences: &[
                    ByteSequence {
                        position_type: PositionType::BOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[0x46, 0x57, 0x53, 0x09])],
                        },
                    },
                    ByteSequence {
                        position_type: PositionType::EOF,
                        offset: Some(0),
                        regex: Regex {
                            tokens: &[Token::Literal(&[0x00, 0x00])],
                        },
                    },
                ],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x43, 0x57, 0x53, 0x09])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
