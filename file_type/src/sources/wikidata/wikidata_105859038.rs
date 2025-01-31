use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859038: FileFormat = FileFormat {
    id: 105_859_038,
    puid: "wikidata/105859038",
    name: "GTA: San Andreas save game (v1.00 PC mod)",
    extensions: &["b"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x83, 0xE5, 0xF3, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
