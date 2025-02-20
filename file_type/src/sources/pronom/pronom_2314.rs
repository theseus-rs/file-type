use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2314: FileType = FileType {
    file_format: &FileFormat {
        id: 2_314,
        source_type: SourceType::Pronom,
        name: "Harvard Graphics Presentation",
        extensions: &["prs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xDC, 0xFE]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x01, 0x00, 0x01, 0x00, 0x00]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x00, 0xC8]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
