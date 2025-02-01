use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853415: FileFormat = FileFormat {
    id: 105_853_415,
    puid: "wikidata/105853415",
    name: "Butcher Signal",
    extensions: &["signal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAB, 0xCD, 0x98, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};
