use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2670: FileType = FileType {
    file_format: &FileFormat {
        id: 2_670,
        source_type: SourceType::Pronom,
        name: "MacCaption File",
        extensions: &["mcc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x69, 0x6C, 0x65, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x3D,
                        0x4D, 0x61, 0x63, 0x43, 0x61, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x5F, 0x4D,
                        0x43, 0x43, 0x20, 0x56, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
