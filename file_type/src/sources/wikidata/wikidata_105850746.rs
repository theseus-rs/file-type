use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850746: FileFormat = FileFormat {
    id: 105_850_746,
    puid: "wikidata/105850746",
    name: "Fullscreen Construction Kit bitmap (448x274)",
    extensions: &["kit"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
