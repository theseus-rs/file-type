use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854201: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_201,
        source_type: SourceType::Wikidata,
        name: "ARP2600V preset",
        extensions: &["arpbank"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x52, 0x50, 0x32, 0x00, 0x04, 0x30, 0x30, 0x30, 0x30, 0x42, 0x41,
                        0x4E, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
