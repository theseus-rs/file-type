use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858601: FileFormat = FileFormat {
    id: 105_858_601,
    source_type: SourceType::Wikidata,
    name: "AliceSoft PMS bitmap",
    extensions: &["pms"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x01, 0x00, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
