use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1460: FileType = FileType {
    file_format: &FileFormat {
        id: 1_460,
        source_type: SourceType::Pronom,
        name: "Sigma RAW Image",
        extensions: &["x3f"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4F, 0x56, 0x62])],
                },
            }],
        }],
        related_formats: &[],
    },
};
