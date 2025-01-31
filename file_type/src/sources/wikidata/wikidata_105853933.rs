use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853933: FileFormat = FileFormat {
    id: 105_853_933,
    puid: "wikidata/105853933",
    name: "DS Squeeze compressed archive",
    extensions: &["ark"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x46, 0x55, 0x47, 0x48, 0x54, 0x41, 0xD5, 0x50,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
