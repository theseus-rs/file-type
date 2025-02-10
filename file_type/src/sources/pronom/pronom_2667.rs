use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2667: FileType = FileType {
    file_format: &FileFormat {
        id: 2_667,
        source_type: SourceType::Pronom,
        name: "Adobe Swatch Exchange",
        extensions: &["ase"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x53, 0x45, 0x46, 0x00, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
