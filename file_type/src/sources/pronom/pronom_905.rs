use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_905: FileType = FileType {
    file_format: &FileFormat {
        id: 905,
        source_type: SourceType::Pronom,
        name: "PrimeOCR",
        extensions: &["pro"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x33, 0x38, 0x30, 0x2C]),
                        Token::SingleWildcard,
                        Token::Literal(&[0x2C]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x2C]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
