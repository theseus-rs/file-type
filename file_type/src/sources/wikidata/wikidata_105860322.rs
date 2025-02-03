use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860322: FileFormat = FileFormat {
    id: 105_860_322,
    source_type: SourceType::Wikidata,
    name: "Build Engine RFF encrypted container",
    extensions: &["rff"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x46, 0x46, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
