use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2162: FileType = FileType {
    file_format: &FileFormat {
        id: 2_162,
        source_type: SourceType::Pronom,
        name: "PTGui Project File",
        extensions: &["pts"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x63, 0x6F, 0x6D, 0x2E, 0x70, 0x74, 0x67, 0x75, 0x69, 0x2E, 0x70, 0x72,
                        0x6F, 0x6A, 0x65, 0x63, 0x74, 0x5F, 0x70, 0x72, 0x6F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
