use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850772: FileFormat = FileFormat {
    id: 105_850_772,
    puid: "wikidata/105850772",
    name: "Fullscreen Construction Kit bitmap (456x274)",
    extensions: &["kil"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
