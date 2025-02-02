use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858465: FileFormat = FileFormat {
    id: 105_858_465,
    source_type: SourceType::Wikidata,
    name: "CyberAIDS infected Apple 2 executable",
    extensions: &["sys"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x13, 0x13, 0x13])],
            },
        }],
    }],
    related_formats: &[],
};
