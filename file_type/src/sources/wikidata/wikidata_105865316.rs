use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865316: FileFormat = FileFormat {
    id: 105_865_316,
    puid: "wikidata/105865316",
    name: "Risen game data archive",
    extensions: &["pak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x00, 0x00, 0x47, 0x33, 0x56, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
