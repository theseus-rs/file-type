use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849708: FileFormat = FileFormat {
    id: 105_849_708,
    puid: "wikidata/105849708",
    name: "Agat Emulator Configuration",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD4, 0x67, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
