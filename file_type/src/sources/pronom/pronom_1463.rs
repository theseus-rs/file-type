use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1463: FileType = FileType {
    file_format: &FileFormat {
        id: 1_463,
        source_type: SourceType::Pronom,
        name: "Gerber Format",
        extensions: &["gbr"],
        media_types: &["application/vnd.gerber"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x46, 0x53, 0x4C, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
