use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857316: FileFormat = FileFormat {
    id: 105_857_316,
    puid: "wikidata/105857316",
    name: "MADS HAG game data archive",
    extensions: &["hag"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x41, 0x44, 0x53, 0x43, 0x4F, 0x4E, 0x43, 0x41, 0x54, 0x20, 0x31, 0x2E,
                    0x30, 0x1A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
