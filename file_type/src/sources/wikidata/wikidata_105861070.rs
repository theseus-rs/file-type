use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861070: FileFormat = FileFormat {
    id: 105_861_070,
    puid: "wikidata/105861070",
    name: "LucaNet archive (v2.0)",
    extensions: &["lnarc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x75, 0x63, 0x61, 0x4E, 0x65, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                    0x74, 0x79, 0x70, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    0x32, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
