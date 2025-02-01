use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853513: FileFormat = FileFormat {
    id: 105_853_513,
    puid: "wikidata/105853513",
    name: "Zoner Draw",
    extensions: &["zmf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5C, 0x00, 0x00, 0x00, 0xDD, 0x5A, 0x01, 0xD4, 0x78, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
