use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853298: FileFormat = FileFormat {
    id: 105_853_298,
    source_type: SourceType::Wikidata,
    name: "Nintendo DS Sound Data",
    extensions: &["sdat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x44, 0x41, 0x54, 0xFF, 0xFE, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
