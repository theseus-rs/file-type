use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_482: FileType = FileType {
    file_format: &FileFormat {
        id: 482,
        source_type: SourceType::Pronom,
        name: "Fractal Image",
        extensions: &["fif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x46, 0x49, 0x46, 0x01]),
                        Token::WildcardCount(52),
                        Token::Literal(&[0xC0]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
