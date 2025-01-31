use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856604: FileFormat = FileFormat {
    id: 105_856_604,
    puid: "wikidata/105856604",
    name: "Wizard 101 game data",
    extensions: &["wad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x49, 0x57, 0x41, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
