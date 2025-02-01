use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852261: FileFormat = FileFormat {
    id: 105_852_261,
    puid: "wikidata/105852261",
    name: "StarCraft 2 game replay",
    extensions: &["sc2replay"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x51, 0x1B])],
            },
        }],
    }],
    related_formats: &[],
};
