use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1248: FileType = FileType {
    file_format: &FileFormat {
        id: 1_248,
        source_type: SourceType::Pronom,
        name: "Verity Collection Index Descriptor File",
        extensions: &["wld", "ddd", "did", "pdd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x74, 0x57, 0x62, 0x63]),
                        Token::WildcardCount(60),
                        Token::Literal(&[0x24, 0x24, 0x24]),
                        Token::WildcardCount(57),
                        Token::Literal(&[0x24, 0x24, 0x66, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
