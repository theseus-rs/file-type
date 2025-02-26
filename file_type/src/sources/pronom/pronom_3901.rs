use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3901: FileType = FileType {
    file_format: &FileFormat {
        id: 3_901,
        source_type: SourceType::Pronom,
        name: "Microsoft Project",
        extensions: &["mpp"],
        media_types: &["application/vnd.ms-project"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x06, 0xF2, 0x00, 0x4A, 0x4C, 0x42, 0x4D, 0x01, 0x00, 0x3E,
                        0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
