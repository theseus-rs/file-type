use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2296: FileType = FileType {
    file_format: &FileFormat {
        id: 2_296,
        source_type: SourceType::Pronom,
        name: "Archimedes Tracker Module",
        extensions: &["musx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x55, 0x53, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
