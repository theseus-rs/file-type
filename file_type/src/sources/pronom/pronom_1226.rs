use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1226: FileType = FileType {
    file_format: &FileFormat {
        id: 1_226,
        source_type: SourceType::Pronom,
        name: "BSDIFF",
        extensions: &["bsdiff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x42, 0x53, 0x44, 0x49, 0x46, 0x46, 0x34, 0x30]),
                        Token::WildcardCount(24),
                        Token::Literal(&[0x42, 0x5A, 0x68]),
                        Token::SingleWildcard,
                        Token::Literal(&[0x31, 0x41, 0x59, 0x26, 0x53, 0x59]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
