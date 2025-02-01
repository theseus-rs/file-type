use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851188: FileFormat = FileFormat {
    id: 105_851_188,
    puid: "wikidata/105851188",
    name: ".NET Micro Framework TinyFont",
    extensions: &["tinyfnt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xA8, 0xB0, 0x95, 0xF9, 0x02, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
