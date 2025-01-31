use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852868: FileFormat = FileFormat {
    id: 105_852_868,
    puid: "wikidata/105852868",
    name: "Scala Multimedia Script (v1.0)",
    extensions: &["script"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x31, 0x2E, 0x30, 0x0A, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
