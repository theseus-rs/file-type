use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1882: FileType = FileType {
    file_format: &FileFormat {
        id: 1_882,
        source_type: SourceType::Pronom,
        name: "AVCHD Movie Object File",
        extensions: &["bdm", "bdmv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4F, 0x42, 0x4A, 0x30, 0x31, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
