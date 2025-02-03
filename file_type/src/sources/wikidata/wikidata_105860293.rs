use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860293: FileFormat = FileFormat {
    id: 105_860_293,
    source_type: SourceType::Wikidata,
    name: "RPG Maker VX encrypted Archive (v2)",
    extensions: &["rgss2a"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x47, 0x53, 0x53, 0x41, 0x44, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
