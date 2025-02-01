use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860458: FileFormat = FileFormat {
    id: 105_860_458,
    puid: "wikidata/105860458",
    name: "Ray Dream Studio",
    extensions: &["rds"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x43, 0x20, 0x7B, 0x0D])],
            },
        }],
    }],
    related_formats: &[],
};
