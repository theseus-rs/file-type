use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856649: FileFormat = FileFormat {
    id: 105_856_649,
    puid: "wikidata/105856649",
    name: "WordStar Screen Font (Wordstar)",
    extensions: &["wsf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x6F, 0x72, 0x64, 0x73, 0x74, 0x61, 0x72, 0x20, 0x53, 0x63, 0x72, 0x65,
                    0x65, 0x6E, 0x20, 0x46, 0x6F, 0x6E, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x00,
                    0x00, 0x90, 0x3C, 0x30, 0x72, 0x06, 0x00, 0x01, 0x00, 0x02, 0x2D, 0x00, 0x2C,
                    0x01, 0x2C, 0x01, 0x01, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
