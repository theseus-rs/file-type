use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1599: FileType = FileType {
    file_format: &FileFormat {
        id: 1_599,
        source_type: SourceType::Pronom,
        name: "WriteNow",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x72, 0x69, 0x74, 0x65, 0x4E, 0x6F, 0x77,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
