use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2511: FileType = FileType {
    file_format: &FileFormat {
        id: 2_511,
        source_type: SourceType::Pronom,
        name: "IntelliFont Font File",
        extensions: &["type", "lib"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x44, 0x00, 0x01, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x00, 0x02]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
