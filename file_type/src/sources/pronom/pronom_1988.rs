use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1988: FileType = FileType {
    file_format: &FileFormat {
        id: 1_988,
        source_type: SourceType::Pronom,
        name: "Synthetic Music Mobile Application Format",
        extensions: &["mmf"],
        media_types: &["application/vnd.yamaha.smaf-audio"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D, 0x4D, 0x44]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x43, 0x4E, 0x54, 0x49]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
