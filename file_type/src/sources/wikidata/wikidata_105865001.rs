use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865001: FileFormat = FileFormat {
    id: 105_865_001,
    source_type: SourceType::Wikidata,
    name: "MegaPaint POP",
    extensions: &["pop"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x50, 0x4F, 0x50, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
