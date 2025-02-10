use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849902: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_902,
        source_type: SourceType::Wikidata,
        name: "Calamus Document",
        extensions: &["cdk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x4D, 0x43, 0x20, 0x43, 0x41, 0x4C, 0x41, 0x4D, 0x55, 0x53, 0x20,
                        0x20, 0x43, 0x44, 0x4B, 0x01, 0x70,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
