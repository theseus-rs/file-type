use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850110: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_110,
        source_type: SourceType::Wikidata,
        name: "16bit DOS COM XcomOR encrypted (v0.99)",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4D, 0x58, 0x66, 0x81, 0x36, 0x03, 0x01, 0x92, 0xEB, 0x04, 0x00,
                        0xEB, 0xF5, 0xD3, 0x5A, 0x81, 0xC2,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
