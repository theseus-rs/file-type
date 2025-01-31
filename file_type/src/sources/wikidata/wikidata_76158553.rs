use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76158553: FileFormat = FileFormat {
    id: 76_158_553,
    puid: "wikidata/76158553",
    name: "MegaPaint VIN",
    extensions: &["vin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x56, 0x49, 0x4E, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
