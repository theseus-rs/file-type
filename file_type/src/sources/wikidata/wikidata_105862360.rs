use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862360: FileFormat = FileFormat {
    id: 105_862_360,
    puid: "wikidata/105862360",
    name: "The Player 6.1a module",
    extensions: &["p61"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x36, 0x31, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
