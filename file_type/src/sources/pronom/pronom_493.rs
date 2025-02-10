use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_493: FileType = FileType {
    file_format: &FileFormat {
        id: 493,
        source_type: SourceType::Pronom,
        name: "JustWrite Text Document",
        extensions: &["jw", "jwt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x46, 0x46, 0x46, 0x49, 0x49, 0x49, 0x49,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
