use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866203: FileFormat = FileFormat {
    id: 105_866_203,
    source_type: SourceType::Wikidata,
    name: "Sun SVR4 package data stream",
    extensions: &["pkg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x50, 0x61, 0x43, 0x6B, 0x41, 0x67, 0x45, 0x20, 0x44, 0x61, 0x54,
                    0x61, 0x53, 0x74, 0x52, 0x65, 0x41, 0x6D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
