use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856188: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_188,
        source_type: SourceType::Wikidata,
        name: "Device Firmare Upgrade format (generic)",
        extensions: &["dfu"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x53, 0x52, 0x2D, 0x64, 0x66, 0x75])],
                },
            }],
        }],
        related_formats: &[],
    },
};
