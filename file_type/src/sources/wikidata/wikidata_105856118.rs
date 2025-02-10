use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856118: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_118,
        source_type: SourceType::Wikidata,
        name: "DeSmuME Firmware Configuration",
        extensions: &["dfc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x65, 0x53, 0x6D, 0x75, 0x4D, 0x45, 0x20, 0x46, 0x69, 0x72, 0x6D,
                        0x77, 0x61, 0x72, 0x65, 0x20, 0x55, 0x73, 0x65, 0x72, 0x20, 0x53, 0x65,
                        0x74, 0x74, 0x69, 0x6E, 0x67, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
