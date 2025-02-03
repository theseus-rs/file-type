use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860430: FileFormat = FileFormat {
    id: 105_860_430,
    source_type: SourceType::Wikidata,
    name: "Borland Reflex Database",
    extensions: &["rxd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x02, 0x33, 0x51, 0x2E, 0x21, 0x26, 0x40, 0x23, 0x24, 0x21, 0x26, 0x26,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
