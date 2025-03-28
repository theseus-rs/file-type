use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1075: FileType = FileType {
    file_format: &FileFormat {
        id: 1_075,
        source_type: SourceType::Pronom,
        name: "Peak Graphical Waveform File",
        extensions: &["pk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xF1, 0x06, 0x00, 0x00, 0x00]),
                        Token::Range(&[0x01], &[0x02]),
                        Token::Literal(&[0x00, 0x00]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x00]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
