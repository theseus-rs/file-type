use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1250: FileType = FileType {
    file_format: &FileFormat {
        id: 1_250,
        source_type: SourceType::Pronom,
        name: "JPM (JPEG 2000 part 6)",
        extensions: &["jpm"],
        media_types: &["image/jpm"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x00, 0x00, 0x00, 0x0C, 0x6A, 0x50, 0x20, 0x20, 0x0D, 0x0A, 0x87, 0x0A,
                        ]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x66, 0x74, 0x79, 0x70, 0x6A, 0x70, 0x6D]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
