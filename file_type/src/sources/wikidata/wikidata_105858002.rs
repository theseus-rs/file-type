use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858002: FileFormat = FileFormat {
    id: 105_858_002,
    source_type: SourceType::Wikidata,
    name: "MAME TACO tape image",
    extensions: &["tap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x41, 0x43, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
