use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861502: FileFormat = FileFormat {
    id: 105_861_502,
    puid: "wikidata/105861502",
    name: "Sprint Layout Printed Circuit Design (v6.0)",
    extensions: &["lay"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x05, 0x33, 0xAA, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
