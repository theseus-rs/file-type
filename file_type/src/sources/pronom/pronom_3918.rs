use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3918: FileType = FileType {
    file_format: &FileFormat {
        id: 3_918,
        source_type: SourceType::Pronom,
        name: "Android Archive File",
        extensions: &["aar"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x5F, 0x54, 0x79, 0x70,
                        0x65, 0x73, 0x5D, 0x2E, 0x78, 0x6D, 0x6C, 0x20, 0xA2,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
