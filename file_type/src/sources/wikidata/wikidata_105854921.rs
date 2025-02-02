use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854921: FileFormat = FileFormat {
    id: 105_854_921,
    source_type: SourceType::Wikidata,
    name: "SMAC compressed data",
    extensions: &["sma"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4D, 0x41, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
