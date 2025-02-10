use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1482: FileType = FileType {
    file_format: &FileFormat {
        id: 1_482,
        source_type: SourceType::Pronom,
        name: "Advanced Function Presentation",
        extensions: &["afp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x5A]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0xD3, 0xA8]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
