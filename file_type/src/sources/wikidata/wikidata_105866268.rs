use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866268: FileFormat = FileFormat {
    id: 105_866_268,
    puid: "wikidata/105866268",
    name: "First Choice Pubblication",
    extensions: &["pub"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAA, 0xAA, 0xAB, 0x00, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
