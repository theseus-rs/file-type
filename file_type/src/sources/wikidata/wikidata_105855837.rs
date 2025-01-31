use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855837: FileFormat = FileFormat {
    id: 105_855_837,
    puid: "wikidata/105855837",
    name: "Dynamic CAD init data",
    extensions: &["dbm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x79, 0x6E, 0x61, 0x6D, 0x69, 0x63, 0x43, 0x41, 0x44, 0x20, 0x69, 0x6E,
                    0x69, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x0A, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
