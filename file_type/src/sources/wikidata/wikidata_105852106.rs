use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852106: FileFormat = FileFormat {
    id: 105_852_106,
    puid: "wikidata/105852106",
    name: "Yu-Gi-Oh! Duel Monsters World Championship savegame",
    extensions: &["sav"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x59, 0x75, 0x47, 0x69, 0x57, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
