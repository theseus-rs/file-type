use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1651: FileType = FileType {
    file_format: &FileFormat {
        id: 1_651,
        source_type: SourceType::Pronom,
        name: "NuFile Exchange Archival Library",
        extensions: &["shk", "sdk", "bxy"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0xF5, 0x46, 0xE9, 0x6C, 0xE5])],
                },
            }],
        }],
        related_formats: &[],
    },
};
