use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856447: FileFormat = FileFormat {
    id: 105_856_447,
    puid: "wikidata/105856447",
    name: "Claw custome level",
    extensions: &["wwd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xF4, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
