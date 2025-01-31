use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860742: FileFormat = FileFormat {
    id: 105_860_742,
    puid: "wikidata/105860742",
    name: "InterBase Relation (v1.0)",
    extensions: &["rel"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x6E, 0x74, 0x65, 0x72, 0x42, 0x61, 0x73, 0x65, 0x20, 0x56, 0x31, 0x2C,
                    0x30, 0x30, 0x20, 0x52, 0x45, 0x4C, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
