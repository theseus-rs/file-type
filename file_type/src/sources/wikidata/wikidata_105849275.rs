use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849275: FileFormat = FileFormat {
    id: 105_849_275,
    puid: "wikidata/105849275",
    name: "TrainController Railroad",
    extensions: &["yrr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0xD1, 0x5D, 0x71, 0x4E, 0xDA, 0xB5, 0xA3,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
