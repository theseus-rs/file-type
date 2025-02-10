use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2451: FileType = FileType {
    file_format: &FileFormat {
        id: 2_451,
        source_type: SourceType::Pronom,
        name: "Art Of Noise",
        extensions: &["aon"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4F, 0x4E, 0x38])],
                },
            }],
        }],
        related_formats: &[],
    },
};
