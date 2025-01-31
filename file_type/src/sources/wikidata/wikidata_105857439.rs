use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857439: FileFormat = FileFormat {
    id: 105_857_439,
    puid: "wikidata/105857439",
    name: "Aladdin 4D Drawing (v2.x)",
    extensions: &["4d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4C, 0x41, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
