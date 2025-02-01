use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860428: FileFormat = FileFormat {
    id: 105_860_428,
    puid: "wikidata/105860428",
    name: "Titanfall 2 game data archive",
    extensions: &["rpak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x50, 0x61, 0x6B, 0x07, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
