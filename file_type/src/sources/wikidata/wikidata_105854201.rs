use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854201: FileFormat = FileFormat {
    id: 105_854_201,
    source_type: SourceType::Wikidata,
    name: "ARP2600V preset",
    extensions: &["arpbank"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x52, 0x50, 0x32, 0x00, 0x04, 0x30, 0x30, 0x30, 0x30, 0x42, 0x41, 0x4E,
                    0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
