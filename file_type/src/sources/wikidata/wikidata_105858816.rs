use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858816: FileFormat = FileFormat {
    id: 105_858_816,
    source_type: SourceType::Wikidata,
    name: "MAKI v1-b bitmap",
    extensions: &["mki"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x4B, 0x49, 0x30, 0x31, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
