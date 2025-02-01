use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849745: FileFormat = FileFormat {
    id: 105_849_745,
    puid: "wikidata/105849745",
    name: "Atari Cartridge",
    extensions: &["car"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x41, 0x52, 0x54, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
