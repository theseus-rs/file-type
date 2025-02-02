use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850657: FileFormat = FileFormat {
    id: 105_850_657,
    source_type: SourceType::Wikidata,
    name: "Fullscreen Construction Kit bitmap (460x274)",
    extensions: &["kid"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x44, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
