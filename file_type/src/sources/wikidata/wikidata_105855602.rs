use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855602: FileFormat = FileFormat {
    id: 105_855_602,
    source_type: SourceType::Wikidata,
    name: "OpenStreetMap O5m data",
    extensions: &["o5m"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xE0, 0x04, 0x6F, 0x35, 0x6D, 0x32, 0xDB, 0x12,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
