use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855970: FileFormat = FileFormat {
    id: 105_855_970,
    puid: "wikidata/105855970",
    name: "Dyalog APL Session",
    extensions: &["dse"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAA, 0x07])],
            },
        }],
    }],
    related_formats: &[],
};
