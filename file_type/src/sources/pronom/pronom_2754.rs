use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2754: FileType = FileType {
    file_format: &FileFormat {
        id: 2_754,
        source_type: SourceType::Pronom,
        name: "Perfect ZX Tape (PZX) Image Format",
        extensions: &["pzx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x5A, 0x58, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
