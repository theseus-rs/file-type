use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856210: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_210,
        source_type: SourceType::Wikidata,
        name: "Device Firmare Upgrade format (v2)",
        extensions: &["dfu"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x53, 0x52, 0x2D, 0x64, 0x66, 0x75, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
