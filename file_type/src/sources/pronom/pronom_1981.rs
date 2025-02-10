use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1981: FileType = FileType {
    file_format: &FileFormat {
        id: 1_981,
        source_type: SourceType::Pronom,
        name: "Alias PowerAnimator File",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(12),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x41, 0x6C, 0x69, 0x61, 0x73, 0x20, 0x50, 0x6F, 0x77, 0x65, 0x72, 0x41,
                            0x6E, 0x69, 0x6D, 0x61, 0x74, 0x6F, 0x72,
                        ]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x39, 0x2E, 0x30]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
