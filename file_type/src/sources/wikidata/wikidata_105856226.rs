use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856226: FileFormat = FileFormat {
    id: 105_856_226,
    source_type: SourceType::Wikidata,
    name: "Dyalog APL Component File",
    extensions: &["dcf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAA, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
