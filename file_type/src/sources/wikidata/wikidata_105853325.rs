use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853325: FileFormat = FileFormat {
    id: 105_853_325,
    puid: "wikidata/105853325",
    name: "Spring Engine Tile",
    extensions: &["smt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x70, 0x72, 0x69, 0x6E, 0x67, 0x20, 0x74, 0x69, 0x6C, 0x65, 0x66, 0x69,
                    0x6C, 0x65, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
