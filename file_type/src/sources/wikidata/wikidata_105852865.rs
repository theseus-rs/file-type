use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852865: FileFormat = FileFormat {
    id: 105_852_865,
    puid: "wikidata/105852865",
    name: "Superbase Program (var 2)",
    extensions: &["sbp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x42, 0x50, 0x50, 0x0A, 0x2F, 0xFF, 0xC2, 0x2F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
