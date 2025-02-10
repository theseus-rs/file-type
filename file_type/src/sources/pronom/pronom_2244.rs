use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2244: FileType = FileType {
    file_format: &FileFormat {
        id: 2_244,
        source_type: SourceType::Pronom,
        name: "MacDraw",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x52, 0x57, 0x47, 0x4D, 0x44, 0x00, 0x06,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
