use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1142: FileType = FileType {
    file_format: &FileFormat {
        id: 1_142,
        source_type: SourceType::Pronom,
        name: "DS_Store File (MAC)",
        extensions: &["ds_store"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x42, 0x75, 0x64, 0x31, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
