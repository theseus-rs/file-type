use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860248: FileFormat = FileFormat {
    id: 105_860_248,
    source_type: SourceType::Wikidata,
    name: "Starcraft Replay",
    extensions: &["rep"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xA7, 0x7E, 0x7E, 0x2B, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x72,
                    0x65, 0x52, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
