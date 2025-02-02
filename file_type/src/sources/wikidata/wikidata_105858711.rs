use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858711: FileFormat = FileFormat {
    id: 105_858_711,
    source_type: SourceType::Wikidata,
    name: "Block Breaker pattern",
    extensions: &["blc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x6C, 0x6F, 0x63, 0x6B, 0x20, 0x42, 0x72, 0x65, 0x61, 0x6B, 0x65, 0x72,
                    0x20, 0x50, 0x61, 0x74, 0x74, 0x65, 0x72, 0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    0x2E, 0x1A, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
