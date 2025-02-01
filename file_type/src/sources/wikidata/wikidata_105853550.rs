use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853550: FileFormat = FileFormat {
    id: 105_853_550,
    puid: "wikidata/105853550",
    name: "ZDA game data archive",
    extensions: &["zda"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x44, 0x41, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
