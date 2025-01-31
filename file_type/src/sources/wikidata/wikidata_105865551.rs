use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865551: FileFormat = FileFormat {
    id: 105_865_551,
    puid: "wikidata/105865551",
    name: "Pecom 64 program",
    extensions: &["pecom"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x2D, 0x36, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
