use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865256: FileFormat = FileFormat {
    id: 105_865_256,
    puid: "wikidata/105865256",
    name: "Pebble Draw Command image",
    extensions: &["pdc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x44, 0x43, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
