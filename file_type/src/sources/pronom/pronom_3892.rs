use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3892: FileType = FileType {
    file_format: &FileFormat {
        id: 3_892,
        source_type: SourceType::Pronom,
        name: "Sony OpenMG Audio",
        extensions: &["oma"],
        media_types: &["audio/ATRAC-ADVANCED-LOSSLESS"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x65, 0x61, 0x33, 0x03]),
                        Token::WildcardCountRange(996, 3_996),
                        Token::Literal(&[0x45, 0x41, 0x33]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x00, 0x60]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x01, 0x0F, 0x50, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
