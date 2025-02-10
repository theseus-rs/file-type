use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854921: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_921,
        source_type: SourceType::Wikidata,
        name: "SMAC compressed data",
        extensions: &["sma"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4D, 0x41, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
