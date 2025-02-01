use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865915: FileFormat = FileFormat {
    id: 105_865_915,
    puid: "wikidata/105865915",
    name: "PPrint Pattern",
    extensions: &["pat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x54, 0x50, 0x2E, 0x50, 0x41, 0x54, 0x54, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
