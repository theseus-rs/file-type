use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2450: FileType = FileType {
    file_format: &FileFormat {
        id: 2_450,
        source_type: SourceType::Pronom,
        name: "Art Of Noise",
        extensions: &["aon"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4F, 0x4E, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
