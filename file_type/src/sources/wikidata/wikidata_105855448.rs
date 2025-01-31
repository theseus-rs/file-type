use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855448: FileFormat = FileFormat {
    id: 105_855_448,
    puid: "wikidata/105855448",
    name: "Final Burn savestate",
    extensions: &["fs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x73, 0x20, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
