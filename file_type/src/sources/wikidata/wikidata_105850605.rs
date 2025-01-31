use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850605: FileFormat = FileFormat {
    id: 105_850_605,
    puid: "wikidata/105850605",
    name: "CoffeeCup Button Factory button",
    extensions: &["cbf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4F, 0x56, 0x49, 0x45, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
