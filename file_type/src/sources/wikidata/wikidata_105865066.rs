use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865066: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_066,
        source_type: SourceType::Wikidata,
        name: "8bit C64 executable Pu-Crunch compressed",
        extensions: &["prg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x08, 0x0B, 0x08, 0xEF, 0x00, 0x9E, 0x32, 0x30, 0x36, 0x31, 0x00,
                        0x00, 0x00, 0x78, 0xEE, 0x30, 0xD0, 0xE6, 0x01, 0xA2, 0x35, 0xBD, 0x42,
                        0x08, 0x9D, 0xFF, 0x01, 0xCA, 0xD0, 0xF7, 0xA2, 0xD6, 0xBD, 0x76, 0x08,
                        0x9D, 0xF6, 0x00, 0xCA, 0xD0, 0xF7, 0xCA, 0xA0,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
