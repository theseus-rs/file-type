use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852098: FileFormat = FileFormat {
    id: 105_852_098,
    puid: "wikidata/105852098",
    name: "Vista Makepath Session (v1.0)",
    extensions: &["ses"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x61, 0x6B, 0x65, 0x70, 0x61, 0x74, 0x68, 0x20, 0x53, 0x65, 0x73, 0x73,
                    0x69, 0x6F, 0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
