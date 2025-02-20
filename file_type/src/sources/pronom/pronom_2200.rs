use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2200: FileType = FileType {
    file_format: &FileFormat {
        id: 2_200,
        source_type: SourceType::Pronom,
        name: "Embedded OpenType (EOT) File Format",
        extensions: &["eot"],
        media_types: &["application/vnd.ms-fontobject"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(8),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x01, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(22),
                        Token::Literal(&[0x4C, 0x50]),
                        Token::WildcardCount(36),
                        Token::Literal(&[0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
