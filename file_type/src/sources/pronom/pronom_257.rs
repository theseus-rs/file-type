use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_257: FileType = FileType {
    file_format: &FileFormat {
        id: 257,
        source_type: SourceType::Pronom,
        name: "Sun Raster Image",
        extensions: &["ras", "sun"],
        media_types: &["image/x-sun-raster"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x59, 0xA6, 0x6A, 0x95]),
                        Token::WildcardCount(8),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::WildcardCount(5),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::SingleWildcard,
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::SingleWildcard,
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
