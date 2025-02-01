use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_79242428: FileFormat = FileFormat {
    id: 79_242_428,
    puid: "wikidata/79242428",
    name: "Antenna Data File",
    extensions: &["adf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x45, 0x56, 0x4E, 0x55, 0x4D, 0x3A, 0x2C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
