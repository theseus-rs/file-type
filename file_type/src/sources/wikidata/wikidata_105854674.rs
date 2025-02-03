use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854674: FileFormat = FileFormat {
    id: 105_854_674,
    source_type: SourceType::Wikidata,
    name: "Alan v3 Compiled adventure",
    extensions: &["a3c"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4C, 0x41, 0x4E, 0x03, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
