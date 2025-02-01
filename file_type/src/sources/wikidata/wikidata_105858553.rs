use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858553: FileFormat = FileFormat {
    id: 105_858_553,
    puid: "wikidata/105858553",
    name: "Electone Registrations",
    extensions: &["b00"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF0, 0x43, 0x70, 0x78, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
