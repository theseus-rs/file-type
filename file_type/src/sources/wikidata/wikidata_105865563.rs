use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865563: FileFormat = FileFormat {
    id: 105_865_563,
    puid: "wikidata/105865563",
    name: "Windows Performance Monitor Alert",
    extensions: &["pma"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xDC, 0x05, 0x83, 0x40, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
