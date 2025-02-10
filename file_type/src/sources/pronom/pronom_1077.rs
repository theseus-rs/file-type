use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1077: FileType = FileType {
    file_format: &FileFormat {
        id: 1_077,
        source_type: SourceType::Pronom,
        name: "ESRI Arc/View Project",
        extensions: &["apr", "def"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x2F, 0x33, 0x2E]),
                        Token::Range(&[0x30], &[0x33]),
                        Token::Literal(&[0x0D, 0x0A, 0x28, 0x4F, 0x44, 0x42, 0x2E, 0x31]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
