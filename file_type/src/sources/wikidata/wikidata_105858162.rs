use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858162: FileFormat = FileFormat {
    id: 105_858_162,
    source_type: SourceType::Wikidata,
    name: "MegaPaint INF",
    extensions: &["inf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x49, 0x4E, 0x46, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
