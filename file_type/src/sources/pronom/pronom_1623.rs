use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1623: FileType = FileType {
    file_format: &FileFormat {
        id: 1_623,
        source_type: SourceType::Pronom,
        name: "P00 C64 Image Format",
        extensions: &["p00", "p01", "p02", "p03", "p04"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x43, 0x36, 0x34, 0x46, 0x69, 0x6C, 0x65, 0x00]),
                        Token::WildcardCount(16),
                        Token::Literal(&[0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
