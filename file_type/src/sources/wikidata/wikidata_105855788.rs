use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855788: FileFormat = FileFormat {
    id: 105_855_788,
    source_type: SourceType::Wikidata,
    name: "Databench form",
    extensions: &["msk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x41, 0x54, 0x41, 0x4D, 0x53, 0x4B, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
