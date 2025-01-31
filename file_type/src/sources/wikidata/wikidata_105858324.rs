use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858324: FileFormat = FileFormat {
    id: 105_858_324,
    puid: "wikidata/105858324",
    name: "ERwin model",
    extensions: &["erwin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0x03, 0x00, 0x00, 0x00, 0x47, 0x44, 0x4D, 0xF8,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
