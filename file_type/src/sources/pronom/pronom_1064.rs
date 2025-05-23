use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1064: FileType = FileType {
    file_format: &FileFormat {
        id: 1_064,
        source_type: SourceType::Pronom,
        name: "ESRI Spatial Index File",
        extensions: &["sbn", "sbx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x00, 0x00, 0x27, 0x0A, 0xFF, 0xFF, 0xFE, 0x70, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00,
                        ]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x00]),
                        Token::Range(&[0x00], &[0x01]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
