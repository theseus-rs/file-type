use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2696: FileType = FileType {
    file_format: &FileFormat {
        id: 2_696,
        source_type: SourceType::Pronom,
        name: "GNU Image Manipulation Program Palette File",
        extensions: &["gpl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x49, 0x4D, 0x50, 0x20, 0x50, 0x61, 0x6C, 0x65, 0x74, 0x74, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
