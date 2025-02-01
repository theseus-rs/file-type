use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862973: FileFormat = FileFormat {
    id: 105_862_973,
    puid: "wikidata/105862973",
    name: "Mini V preset",
    extensions: &["minibank"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x49, 0x4E, 0x49, 0x00, 0x04, 0x30, 0x30, 0x30, 0x30, 0x42, 0x41, 0x4E,
                    0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
