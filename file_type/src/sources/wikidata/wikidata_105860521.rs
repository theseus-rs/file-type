use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860521: FileFormat = FileFormat {
    id: 105_860_521,
    puid: "wikidata/105860521",
    name: "Etherlords 2 game data archive",
    extensions: &["res"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0xE2, 0x9C, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
