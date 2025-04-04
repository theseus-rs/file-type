use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855987: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_987,
        source_type: SourceType::Wikidata,
        name: "InterBase Data (v1.0)",
        extensions: &["dta"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x6E, 0x74, 0x65, 0x72, 0x42, 0x61, 0x73, 0x65, 0x20, 0x56, 0x31,
                        0x2C, 0x30, 0x30, 0x20, 0x44, 0x54, 0x41, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
