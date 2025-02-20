use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1780: FileType = FileType {
    file_format: &FileFormat {
        id: 1_780,
        source_type: SourceType::Pronom,
        name: "Jamcracker Tracker Module",
        extensions: &["jam"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x65, 0x45, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
