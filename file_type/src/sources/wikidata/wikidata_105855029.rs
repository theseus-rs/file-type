use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855029: FileFormat = FileFormat {
    id: 105_855_029,
    puid: "wikidata/105855029",
    name: "CS-80V preset",
    extensions: &["ays"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x53, 0x38, 0x30, 0x00, 0x04, 0x30, 0x30, 0x30, 0x30, 0x42, 0x41, 0x4E,
                    0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
