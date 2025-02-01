use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853093: FileFormat = FileFormat {
    id: 105_853_093,
    puid: "wikidata/105853093",
    name: "Scala Multimedia Script (v3.0)",
    extensions: &["script"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x33, 0x2E, 0x30, 0x0A, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
