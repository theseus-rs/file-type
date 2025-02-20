use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2246: FileType = FileType {
    file_format: &FileFormat {
        id: 2_246,
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
                        0x64, 0x44, 0x6F, 0x63, 0x44, 0x32, 0xFF, 0xFF,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
