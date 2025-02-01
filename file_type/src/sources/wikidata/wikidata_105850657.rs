use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850657: FileFormat = FileFormat {
    id: 105_850_657,
    puid: "wikidata/105850657",
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
