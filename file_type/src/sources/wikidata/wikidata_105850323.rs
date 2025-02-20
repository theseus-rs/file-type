use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850323: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_323,
        source_type: SourceType::Wikidata,
        name: "16bit COM executable BAT2EXEC v1.3",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFC, 0xBD, 0x20, 0x01, 0x8B, 0x6E, 0x00, 0x8B, 0xA6, 0x02, 0x00, 0x8B,
                        0x9E, 0x04, 0x00, 0xB4, 0x4A, 0xCD, 0x21, 0xA1, 0x2C, 0x00, 0x89, 0x86,
                        0x1A, 0x00, 0x8B, 0x9E, 0x00, 0x00, 0xFF, 0xE3,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
