use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855294: FileFormat = FileFormat {
    id: 105_855_294,
    puid: "wikidata/105855294",
    name: "Runes of Magic game data archive",
    extensions: &["fdb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x42, 0x44, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
