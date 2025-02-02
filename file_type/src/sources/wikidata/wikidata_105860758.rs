use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860758: FileFormat = FileFormat {
    id: 105_860_758,
    source_type: SourceType::Wikidata,
    name: "Rathole compressed data",
    extensions: &["rhl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x48, 0x4C, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
