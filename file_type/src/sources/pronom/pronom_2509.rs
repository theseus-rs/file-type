use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2509: FileType = FileType {
    file_format: &FileFormat {
        id: 2_509,
        source_type: SourceType::Pronom,
        name: "ZBrush MatCap",
        extensions: &["zmt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x42, 0x72, 0x75, 0x73, 0x68, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
