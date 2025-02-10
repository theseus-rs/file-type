use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856380: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_380,
        source_type: SourceType::Wikidata,
        name: "PEC service info (daticert.xml)",
        extensions: &["xml"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0xF5, 0xF5, 0x50, 0x09, 0x5F, 0xD8, 0xF9, 0x31, 0xCB, 0x78, 0x65,
                        0x98, 0x83, 0x2C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
