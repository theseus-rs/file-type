use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854764: FileFormat = FileFormat {
    id: 105_854_764,
    puid: "wikidata/105854764",
    name: "Big Mutha Truckers 2 game data Archive",
    extensions: &["ar"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x52, 0x43, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
