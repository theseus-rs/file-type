use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860056: FileFormat = FileFormat {
    id: 105_860_056,
    source_type: SourceType::Wikidata,
    name: "MegaPaint Vector format",
    extensions: &["vek"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x56, 0x45, 0x4B, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
