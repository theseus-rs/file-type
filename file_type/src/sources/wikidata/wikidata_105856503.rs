use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856503: FileFormat = FileFormat {
    id: 105_856_503,
    puid: "wikidata/105856503",
    name: "Wordwall data",
    extensions: &["wwf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0F, 0x57, 0x4F, 0x52, 0x44, 0x57, 0x41, 0x4C, 0x4C, 0x44, 0x41, 0x54, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
