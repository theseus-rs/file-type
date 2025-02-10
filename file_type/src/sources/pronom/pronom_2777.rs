use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2777: FileType = FileType {
    file_format: &FileFormat {
        id: 2_777,
        source_type: SourceType::Pronom,
        name: "Revolution Stack",
        extensions: &["rev", "livecode"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x45, 0x56, 0x4F, 0x32, 0x37, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
