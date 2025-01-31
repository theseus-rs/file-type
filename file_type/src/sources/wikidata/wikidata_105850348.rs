use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850348: FileFormat = FileFormat {
    id: 105_850_348,
    puid: "wikidata/105850348",
    name: "Calamus Farb (Colour) Table",
    extensions: &["cft"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x4D, 0x43, 0x20, 0x43, 0x41, 0x4C, 0x41, 0x4D, 0x55, 0x53, 0x20, 0x20,
                    0x4C, 0x55, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
