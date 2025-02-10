use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1308: FileType = FileType {
    file_format: &FileFormat {
        id: 1_308,
        source_type: SourceType::Pronom,
        name: "Adobe Multiple Master Metrics font file",
        extensions: &["mmm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x0A, 0x01, 0x01, 0x00, 0x43, 0x6F, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68,
                            0x74, 0x20,
                        ]),
                        Token::WildcardCount(158),
                        Token::Literal(&[
                            0xFF, 0x00, 0x75, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
                            0x76, 0x00, 0x00, 0x00, 0xE8, 0x03, 0xE8, 0x03,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
