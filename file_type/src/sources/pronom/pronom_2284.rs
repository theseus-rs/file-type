use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2284: FileType = FileType {
    file_format: &FileFormat {
        id: 2_284,
        source_type: SourceType::Pronom,
        name: "Autorun Maestro Menu File",
        extensions: &["mnu"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x45, 0x4E, 0x55, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
