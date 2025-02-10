use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1028: FileType = FileType {
    file_format: &FileFormat {
        id: 1_028,
        source_type: SourceType::Pronom,
        name: "Microsoft Front Page Server Extension Configuration",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x76, 0x74, 0x69, 0x5F, 0x65, 0x6E, 0x63, 0x6F, 0x64, 0x69, 0x6E, 0x67,
                        0x3A, 0x53, 0x52, 0x7C, 0x75, 0x74, 0x66, 0x38, 0x2D, 0x6E, 0x6C, 0x0D,
                        0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
