use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861326: FileFormat = FileFormat {
    id: 105_861_326,
    puid: "wikidata/105861326",
    name: "64LAN container",
    extensions: &["l64"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x36, 0x34, 0x4C, 0x41, 0x4E, 0x20, 0x49, 0x44, 0x42, 0x4C, 0x4F, 0x43, 0x4B,
                    0x20, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
