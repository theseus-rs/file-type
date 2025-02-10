use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2120: FileType = FileType {
    file_format: &FileFormat {
        id: 2_120,
        source_type: SourceType::Pronom,
        name: "PrintMaster Gold Project",
        extensions: &["ban", "cal", "car", "let", "sig"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4C, 0x53, 0x44, 0x4F, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
