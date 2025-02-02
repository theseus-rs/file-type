use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860201: FileFormat = FileFormat {
    id: 105_860_201,
    source_type: SourceType::Wikidata,
    name: "Twist Report script",
    extensions: &["r"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x45, 0x50, 0x4F, 0x52, 0x54, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
