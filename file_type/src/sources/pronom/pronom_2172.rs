use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2172: FileType = FileType {
    file_format: &FileFormat {
        id: 2_172,
        source_type: SourceType::Pronom,
        name: "QuickBooks Backup File",
        extensions: &["qbb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x86, 0x00, 0x00, 0x06, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
