use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_328: FileType = FileType {
    file_format: &FileFormat {
        id: 328,
        source_type: SourceType::Pronom,
        name: "ESRI Arc/View ShapeFile",
        extensions: &["shp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x00, 0x00, 0x27, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        ]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0xE8, 0x03, 0x00, 0x00]),
                        Token::WildcardCount(68),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x01]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
