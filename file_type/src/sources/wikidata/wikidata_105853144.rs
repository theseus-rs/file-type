use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853144: FileFormat = FileFormat {
    id: 105_853_144,
    puid: "wikidata/105853144",
    name: "NTRQ module",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x54, 0x52, 0x51])],
            },
        }],
    }],
    related_formats: &[],
};
