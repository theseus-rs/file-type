use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855467: FileFormat = FileFormat {
    id: 105_855_467,
    puid: "wikidata/105855467",
    name: "Faery Tale Adventure 2 save game",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x54, 0x41, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
