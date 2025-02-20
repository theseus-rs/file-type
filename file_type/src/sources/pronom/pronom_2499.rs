use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2499: FileType = FileType {
    file_format: &FileFormat {
        id: 2_499,
        source_type: SourceType::Pronom,
        name: "YAODL (Yet Another Object Description Language) File",
        extensions: &["ydl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x59, 0x41, 0x4F, 0x44, 0x4C, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
