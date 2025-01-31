use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856449: FileFormat = FileFormat {
    id: 105_856_449,
    puid: "wikidata/105856449",
    name: "WordStar Screen Font (Award)",
    extensions: &["wsf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x57, 0x41, 0x52, 0x44, 0x20, 0x53, 0x4F, 0x46, 0x54, 0x57, 0x41, 0x52,
                    0x45, 0x2C, 0x20, 0x49, 0x4E, 0x43, 0x2E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02, 0x2D, 0x00, 0x2C,
                    0x01, 0x2C, 0x01, 0x01, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
