use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857119: FileFormat = FileFormat {
    id: 105_857_119,
    puid: "wikidata/105857119",
    name: "Mind Games - Gomoku saved game",
    extensions: &["gam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x6F, 0x6D, 0x6F, 0x6B, 0x75, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
