use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853252: FileFormat = FileFormat {
    id: 105_853_252,
    puid: "wikidata/105853252",
    name: "Roland Fantom audio",
    extensions: &["svd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x6E, 0x53, 0x56, 0x44, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
