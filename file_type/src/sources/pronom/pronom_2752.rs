use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2752: FileType = FileType {
    file_format: &FileFormat {
        id: 2_752,
        source_type: SourceType::Pronom,
        name: "Nokia Picture Message",
        extensions: &["npm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x50, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
